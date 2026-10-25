import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Folder of the project's agent types. */
const AGENTS = join(import.meta.dirname, "..", ".claude", "agents");

/** The model each agent type runs on, as decided at F20's second round (EN-066, 2026-10-17). */
const MODELS: Readonly<Record<string, string>> = {
  auditor: "opus",
  "functional-analyst": "opus",
  "math-expert": "opus",
  researcher: "opus",
  "story-worker": "opus",
};

/** Words naming each model in the line that gives its reason. */
const MODEL_NAMES: Readonly<Record<string, string>> = { opus: "Opus", sonnet: "Sonnet" };

/**
 * The `model` field of an agent file's front matter.
 *
 * @param text - the agent file
 * @returns its value, or an empty string when the field is missing
 */
function modelOf(text: string): string {
  return /^model: (?<model>\S+)$/mu.exec(text)?.groups?.["model"] ?? "";
}

describe("agent models", () => {
  const files = readdirSync(AGENTS).filter((name) => name.endsWith(".md"));

  it("[F20.AC2] knows every agent type of the folder", () => {
    expect(
      files
        .map((name) => name.replace(/\.md$/u, ""))
        .toSorted((left, right) => left.localeCompare(right)),
    ).toEqual(Object.keys(MODELS).toSorted((left, right) => left.localeCompare(right)));
  });

  it.each(files)("[F20.AC2] %s names its model and why", (name) => {
    const text = readFileSync(join(AGENTS, name), "utf8");
    const model = MODELS[name.replace(/\.md$/u, "")] ?? "";

    expect(modelOf(text)).toBe(model);
    expect(text).toContain(`Why ${MODEL_NAMES[model] ?? ""} (EN-066): `);
  });
});
