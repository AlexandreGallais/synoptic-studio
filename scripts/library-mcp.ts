// MCP server of the local source library (docs/tooling/sources.md, ADR-0039): three tools over
// the stdio transport, newline-delimited JSON-RPC (`REF-MCP-TRANSPORTS`, `REF-MCP-TOOLS`),
// written without the SDK. Declared in `.mcp.json`; the same functions back `npm run library`.

import { createInterface } from "node:readline";

import { libraryPage, tableOfContents } from "./library-pages.ts";
import { libraryPath, openLibrary, searchLibrary } from "./library-store.ts";
import { projectOf, recordUse } from "./library-usage.ts";

import type { DatabaseSync } from "node:sqlite";

/** A call of one of the library's tools. */
export type ToolCall = {
  /** Its arguments. */
  readonly input: unknown;
  /** `library_search`, `library_get` or `library_read`. */
  readonly name: string;
  /** The project that asks (`projectOf`). */
  readonly project: string;
};

/** Protocol version this server speaks. */
const PROTOCOL_VERSION = "2025-06-18";

/** JSON-RPC error code of an unknown method. */
const METHOD_NOT_FOUND = -32_601;

/** Most sections a search returns when the call does not say. */
const DEFAULT_LIMIT = 8;

/** Tools of the server, as `tools/list` describes them. */
export const TOOLS = [
  {
    description:
      "Search the local library of documentation pages already read (full text, best sections first). Use before any web search or fetch.",
    inputSchema: {
      properties: {
        limit: { description: "most sections returned (default 8)", type: "number" },
        query: { description: "words to find", type: "string" },
      },
      required: ["query"],
      type: "object",
    },
    name: "library_search",
  },
  {
    description:
      "Get a documentation page through the local library: stored and fresh, it is read from there; otherwise fetched online and stored. Returns its table of contents; read sections with library_read.",
    inputSchema: {
      properties: {
        refresh: { description: "read it online whatever its age", type: "boolean" },
        url: { description: "address of the page", type: "string" },
      },
      required: ["url"],
      type: "object",
    },
    name: "library_get",
  },
  {
    description:
      "Read sections of a page stored in the local library, by their numbers in its table of contents.",
    inputSchema: {
      properties: {
        sections: { description: "section numbers", items: { type: "number" }, type: "array" },
        url: { description: "address of the page", type: "string" },
      },
      required: ["url", "sections"],
      type: "object",
    },
    name: "library_read",
  },
];

/**
 * Property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property, or `undefined` when `value` is not an object
 */
function field(value: unknown, key: string): unknown {
  return typeof value === "object" && value !== null ? Reflect.get(value, key) : undefined;
}

/**
 * String property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a string, else an empty string
 */
function text(value: unknown, key: string): string {
  const property = field(value, key);

  return typeof property === "string" ? property : "";
}

/**
 * Answer of `library_search`.
 *
 * @param database - open library
 * @param project - the project that asks
 * @param input - arguments: `query`, optional `limit`
 * @returns one line per section found, with its address, number, heading and snippet
 */
function searchAnswer(database: DatabaseSync, project: string, input: unknown): string {
  const limit = field(input, "limit");
  const hits = searchLibrary(
    database,
    text(input, "query"),
    typeof limit === "number" ? limit : DEFAULT_LIMIT,
  );
  const lines = hits.map(
    (hit) => `${hit.url} §${String(hit.index)} — ${hit.title} › ${hit.heading}\n  ${hit.snippet}`,
  );
  const use = {
    origin: hits.length > 0 ? "found" : "none",
    project,
    query: text(input, "query"),
    tool: "search",
    url: "",
  };

  recordUse(database, use, Temporal.Now.instant().epochMilliseconds);

  return lines.length === 0 ? "No section found: get the page with library_get." : lines.join("\n");
}

/**
 * Answer of `library_read`.
 *
 * @param database - open library
 * @param project - the project that asks
 * @param input - arguments: `url`, `sections` (numbers)
 * @returns the text of the sections asked, in page order
 */
async function readAnswer(
  database: DatabaseSync,
  project: string,
  input: unknown,
): Promise<string> {
  const asked = field(input, "sections");
  const wanted = new Set(Array.isArray(asked) ? asked.map(Number) : []);
  const entry = await libraryPage(database, text(input, "url"), false);

  recordUse(
    database,
    { origin: entry.origin, project, query: "", tool: "read", url: entry.page.url },
    Temporal.Now.instant().epochMilliseconds,
  );

  return entry.sections
    .filter((section) => wanted.has(section.index))
    .map((section) => section.text)
    .join("\n\n");
}

/**
 * Answer of `library_get`.
 *
 * @param database - open library
 * @param project - the project that asks
 * @param input - arguments: `url`, optional `refresh`
 * @returns the page's table of contents
 */
async function getAnswer(database: DatabaseSync, project: string, input: unknown): Promise<string> {
  const entry = await libraryPage(database, text(input, "url"), field(input, "refresh") === true);

  recordUse(
    database,
    { origin: entry.origin, project, query: "", tool: "get", url: entry.page.url },
    Temporal.Now.instant().epochMilliseconds,
  );

  return tableOfContents(entry);
}

/**
 * Text answer of a tool.
 *
 * @param database - open library
 * @param call - the tool, its arguments and the project that asks, recorded with the use
 * @returns the text to give back
 */
export async function callTool(database: DatabaseSync, call: ToolCall): Promise<string> {
  const { input, name, project } = call;

  if (name === "library_search") {
    return searchAnswer(database, project, input);
  }

  return name === "library_get"
    ? await getAnswer(database, project, input)
    : await readAnswer(database, project, input);
}

/**
 * Result of a request other than a tool call.
 *
 * @param method - JSON-RPC method
 * @returns its result, or `undefined` for an unknown method
 */
function protocolResult(method: string): unknown {
  const results = new Map<string, unknown>([
    [
      "initialize",
      {
        capabilities: { tools: {} },
        protocolVersion: PROTOCOL_VERSION,
        serverInfo: { name: "library", version: "1.0.0" },
      },
    ],
    ["ping", {}],
    ["tools/list", { tools: TOOLS }],
  ]);

  return results.get(method);
}

/**
 * Result of a tool call: its text, or the error it met. A `library_get` that met one, its fetch
 * failing before any answer (a name, a certificate, a time-out), is recorded as `failed`, so that
 * the page hook lets the page be read otherwise (AUD-022).
 *
 * @param database - open library
 * @param project - the project that asks
 * @param parameters - `name` and `arguments` of the call
 * @returns the MCP tool result
 */
async function toolResult(
  database: DatabaseSync,
  project: string,
  parameters: unknown,
): Promise<unknown> {
  const input = field(parameters, "arguments");
  const name = text(parameters, "name");

  try {
    const answerText = await callTool(database, { input, name, project });

    return { content: [{ text: answerText, type: "text" }], isError: false };
  } catch (error) {
    if (name === "library_get") {
      recordUse(
        database,
        { origin: "failed", project, query: "", tool: "get", url: text(input, "url") },
        Temporal.Now.instant().epochMilliseconds,
      );
    }

    return { content: [{ text: String(error), type: "text" }], isError: true };
  }
}

/**
 * Answer to one JSON-RPC message, or nothing for a notification.
 *
 * @param database - open library
 * @param project - the project the server runs for
 * @param message - parsed request
 * @returns the response object, `undefined` for a notification
 */
export async function answer(
  database: DatabaseSync,
  project: string,
  message: unknown,
): Promise<unknown> {
  const id = field(message, "id");

  if (id === undefined) {
    return undefined;
  }

  const method = text(message, "method");
  const result =
    method === "tools/call"
      ? await toolResult(database, project, field(message, "params"))
      : protocolResult(method);

  return result === undefined
    ? {
        error: { code: METHOD_NOT_FOUND, message: `Unknown method: ${method}` },
        id,
        jsonrpc: "2.0",
      }
    : { id, jsonrpc: "2.0", result };
}

/**
 * Answers each line of the standard input on the standard output, until the input closes.
 *
 * @param database - open library
 * @param project - the project the server runs for
 * @returns once the client has closed the input
 */
async function serve(database: DatabaseSync, project: string): Promise<void> {
  const lines = createInterface({ input: process.stdin });

  for await (const line of lines) {
    const response =
      line.trim() === "" ? undefined : await answer(database, project, JSON.parse(line));

    if (response !== undefined) {
      process.stdout.write(`${JSON.stringify(response)}\n`);
    }
  }
}

if (import.meta.main) {
  // Claude Code starts the server in the project's folder; `CLAUDE_PROJECT_DIR` names it when set.
  await serve(
    openLibrary(libraryPath()),
    projectOf(process.env["CLAUDE_PROJECT_DIR"] ?? process.cwd()),
  );
}
