import { describe, expect, it } from "vitest";

import config from "../vite.config";

describe("Vitest's workers (EN-047)", () => {
  it("[F18.AC20] uses half the machine's cores, so that worktrees testing at once share it", () => {
    const resolved = config({ command: "serve", mode: "test" });

    expect(resolved.test?.maxWorkers).toBe("50%");
  });
});
