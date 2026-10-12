// Events of the agent's work read from Claude Code transcripts (docs/tooling/metrics.md): each
// model request with its tokens, each tool call with its start, end and outcome. Every line of a
// transcript carries its time, session and Git branch.

/** Tokens of one model request, as its `usage` reports them. */
type Tokens = {
  readonly cacheRead: number;
  readonly cacheWrite: number;
  readonly input: number;
  readonly output: number;
};

/** One model request. */
export type ModelRequest = {
  readonly branch: string;
  readonly id: string;
  readonly session: string;
  readonly time: number;
  readonly tokens: Tokens;
};

/** One tool call, from its request to its result. */
export type ToolCall = {
  readonly branch: string;
  readonly end: number;
  readonly id: string;
  readonly isOk: boolean;
  readonly session: string;
  readonly start: number;
  readonly summary: string;
  readonly tool: string;
};

/** What one transcript holds. */
export type TranscriptEvents = {
  readonly calls: readonly ToolCall[];
  readonly requests: readonly ModelRequest[];
};

/** Fields shared by every line of a transcript. */
type LineContext = {
  readonly branch: string;
  readonly session: string;
  readonly time: number;
};

/**
 * Property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property, or `undefined` when `value` is not an object
 */
export function field(value: unknown, key: string): unknown {
  return typeof value === "object" && value !== null ? Reflect.get(value, key) : undefined;
}

/**
 * String property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a string, else an empty string
 */
export function text(value: unknown, key: string): string {
  const property = field(value, key);

  return typeof property === "string" ? property : "";
}

/** Input keys naming what a tool acted on, in order of preference. */
const SUBJECT_KEYS = ["command", "file_path", "pattern", "url", "query", "skill", "description"];

/**
 * What a tool call acted on, on one line: its command, file, pattern, address, query or task.
 *
 * @param input - the arguments the tool received (`tool_use.input`)
 * @returns the first non-empty one, white space collapsed
 */
export function callSummary(input: unknown): string {
  return (
    SUBJECT_KEYS.map((key) => text(input, key)).find((value) => value !== "") ?? ""
  ).replaceAll(/\s+/gu, " ");
}

/**
 * Number property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a number, else 0
 */
export function count(value: unknown, key: string): number {
  const property = field(value, key);

  return typeof property === "number" ? property : 0;
}

/**
 * Content blocks of a transcript line's message.
 *
 * @param line - parsed transcript line
 * @returns its `message.content` when it is a list, else an empty list
 */
function blocks(line: unknown): readonly unknown[] {
  const content = field(field(line, "message"), "content");

  return Array.isArray(content) ? content : [];
}

/**
 * Tokens of a request's `usage`.
 *
 * @param usage - the `message.usage` object of an assistant line
 * @returns its four token counts
 */
function tokensOf(usage: unknown): Tokens {
  return {
    cacheRead: count(usage, "cache_read_input_tokens"),
    cacheWrite: count(usage, "cache_creation_input_tokens"),
    input: count(usage, "input_tokens"),
    output: count(usage, "output_tokens"),
  };
}

/**
 * Parsed lines of a transcript with their context, lines without a time left out.
 *
 * @param transcript - text of a JSON Lines transcript
 * @returns each line with its time (epoch milliseconds), session and branch
 */
function contextLines(transcript: string): readonly (readonly [unknown, LineContext])[] {
  return transcript
    .split("\n")
    .filter((line) => line.startsWith("{"))
    .map((line): unknown => JSON.parse(line))
    .filter((line) => text(line, "timestamp") !== "")
    .map((line) => [
      line,
      {
        branch: text(line, "gitBranch"),
        session: text(line, "sessionId"),
        time: Temporal.Instant.from(text(line, "timestamp")).epochMilliseconds,
      },
    ]);
}

/**
 * Results of the tool calls of a transcript.
 *
 * @param lines - parsed lines with their context
 * @returns when each result came and whether it was an error, by tool use id
 */
function toolResults(
  lines: readonly (readonly [unknown, LineContext])[],
): ReadonlyMap<string, { readonly end: number; readonly isOk: boolean }> {
  return new Map(
    lines.flatMap(([line, context]) =>
      blocks(line)
        .filter((block) => text(block, "type") === "tool_result")
        .map((block) => [
          text(block, "tool_use_id"),
          { end: context.time, isOk: field(block, "is_error") !== true },
        ]),
    ),
  );
}

/**
 * Model requests and tool calls of one transcript.
 *
 * A request written over several lines (one per content block) is counted once; a tool call takes
 * the time of its request as start and the time of its result as end.
 *
 * @param transcript - text of a JSON Lines transcript
 * @returns its requests and its answered tool calls
 */
export function transcriptEvents(transcript: string): TranscriptEvents {
  const lines = contextLines(transcript);
  const assistant = lines.filter(([line]) => text(line, "type") === "assistant");
  const requests = new Map(
    assistant.map(([line, context]) => {
      const message = field(line, "message");

      return [
        text(message, "id"),
        { ...context, id: text(message, "id"), tokens: tokensOf(field(message, "usage")) },
      ];
    }),
  );
  const results = toolResults(lines);
  const calls = assistant.flatMap(([line, context]) =>
    blocks(line)
      .filter((block) => text(block, "type") === "tool_use" && results.has(text(block, "id")))
      .map((block) => ({
        branch: context.branch,
        end: results.get(text(block, "id"))?.end ?? context.time,
        id: text(block, "id"),
        isOk: results.get(text(block, "id"))?.isOk ?? true,
        session: context.session,
        start: context.time,
        summary: callSummary(field(block, "input")),
        tool: text(block, "name"),
      })),
  );

  return {
    calls,
    requests: requests
      .values()
      .filter((request) => request.id !== "")
      .toArray(),
  };
}

/**
 * Whether a transcript line is a prompt typed by the person, not a tool result nor a reminder.
 *
 * @param line - parsed transcript line
 * @returns true for a `user` line holding text
 */
function isPrompt(line: unknown): boolean {
  const content = field(field(line, "message"), "content");

  return (
    text(line, "type") === "user" &&
    field(line, "isMeta") !== true &&
    (typeof content === "string" || blocks(line).some((block) => text(block, "type") === "text"))
  );
}

/**
 * When the current turn started: the first prompt after the agent last ended its turn. Messages
 * the person sends while the agent works stay inside the turn.
 *
 * @param transcript - text of a JSON Lines transcript
 * @returns epoch milliseconds, or 0 when there is no prompt
 */
export function turnStartTime(transcript: string): number {
  return contextLines(transcript).reduce(
    (state, [line, context]) => {
      if (text(field(line, "message"), "stop_reason") === "end_turn") {
        return { isWaiting: true, start: state.start };
      }

      return state.isWaiting && isPrompt(line) ? { isWaiting: false, start: context.time } : state;
    },
    { isWaiting: true, start: 0 },
  ).start;
}
