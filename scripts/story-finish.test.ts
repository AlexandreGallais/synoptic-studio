import { describe, expect, it } from "vitest";

import { nextStep, rowsMessage } from "./story-finish";

describe("story-finish", () => {
  it("cleans once merged, waits while open for three minutes, then stops", () => {
    expect(nextStep("MERGED", 1)).toBe("clean");
    expect(nextStep("OPEN", 17)).toBe("wait");
    expect(nextStep("OPEN", 18)).toBe("stop");
    expect(nextStep("CLOSED", 1)).toBe("stop");
  });

  it("names the merged story in the commit moving its rows into the logs", () => {
    expect(rowsMessage("chore/en-050-shared-files-written-at-merge")).toBe(
      "docs(docs): move the rows of EN-050 into the logs\n\nRefs: EN-050\n",
    );

    expect(rowsMessage("feat/us-061-font-list")).toBe(
      "docs(docs): move the rows of US-061 into the logs\n\nRefs: US-061\n",
    );
  });
});
