// Command line of the local source library (docs/tooling/sources.md):
//   npm run library -- search <words>      sections best matching the words
//   npm run library -- get <url> [--refresh] table of contents, fetched when missing or stale
//   npm run library -- read <url> <n> [<n>…] sections of a page
//   npm run library -- list                 stored pages, last read first
//   npm run library -- stats [YYYY-MM-DD]   sources consulted most, searches, share served locally

import { callTool } from "./library-mcp.ts";
import { libraryPath, listPages, openLibrary } from "./library-store.ts";
import { usageReport } from "./library-usage.ts";

if (import.meta.main) {
  const [command = "", ...rest] = process.argv.slice(2);
  const database = openLibrary(libraryPath());
  const words = rest.filter((word) => !word.startsWith("--"));
  const inputs = new Map<string, unknown>([
    ["get", { refresh: rest.includes("--refresh"), url: words[0] }],
    ["read", { sections: words.slice(1).map(Number), url: words[0] }],
    ["search", { query: words.join(" ") }],
  ]);

  if (command === "stats") {
    const since =
      words[0] === undefined ? 0 : Temporal.Instant.from(`${words[0]}T00:00:00Z`).epochMilliseconds;

    process.stdout.write(`${usageReport(database, since)}\n`);
  } else if (command === "list") {
    const lines = listPages(database).map((page) => {
      const day = Temporal.Instant.fromEpochMilliseconds(page.fetchedAt).toZonedDateTimeISO("UTC");

      return `${day.toPlainDate().toString()}  ${page.url}  ${page.title}`;
    });

    process.stdout.write(`${lines.join("\n")}\n`);
  } else if (inputs.has(command)) {
    process.stdout.write(
      `${await callTool(database, `library_${command}`, inputs.get(command))}\n`,
    );
  } else {
    process.stderr.write(
      "Usage: npm run library -- search <words> | get <url> [--refresh] | read <url> <n>… | list | stats [since]\n",
    );

    process.exitCode = 1;
  }
}
