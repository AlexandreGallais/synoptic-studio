// Tokens of a session per model that answered (EN-066), for the session report: which share of a
// run went to Opus and which to Sonnet, now that each agent type names its model.

import type { TranscriptEvents } from "./metrics-events.ts";

/**
 * Tokens of every kind per model that answered, the most first; a model without tokens left out.
 *
 * @param events - requests and tool calls
 * @returns each model's id and its tokens: cache reads and writes, input and output
 */
export function tokensByModel(events: TranscriptEvents): readonly (readonly [string, number])[] {
  const totals = new Map<string, number>();

  for (const { model, tokens } of events.requests) {
    totals.set(
      model,
      (totals.get(model) ?? 0) +
        tokens.cacheRead +
        tokens.cacheWrite +
        tokens.input +
        tokens.output,
    );
  }

  return [...totals]
    .filter(([, total]) => total > 0)
    .toSorted(([, left], [, right]) => right - left);
}

/**
 * The tokens of each model on one line, its id without the `claude-` every one starts with.
 *
 * @param totals - tokens per model, the most first
 * @returns e.g. `opus-5-5 1,253 · sonnet-5-5 279`; `unknown` for a request naming no model;
 * `none` without a request
 */
export function modelTokens(totals: readonly (readonly [string, number])[]): string {
  return totals.length === 0
    ? "none"
    : totals
        .map(
          ([model, total]) =>
            `${model === "" ? "unknown" : model.replace(/^claude-/u, "")} ${total.toLocaleString("en-US")}`,
        )
        .join(" · ");
}
