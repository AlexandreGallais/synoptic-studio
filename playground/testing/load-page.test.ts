// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { loadPage } from "./load-page";

describe("loadPage", () => {
  it("puts the app's page in the document, not mounted yet, at the address asked", () => {
    // The test mounts the playground itself: nothing is drawn before the app's first change
    // detection.
    loadPage("?step=3");

    expect(document.querySelector("#canvas")).not.toBeNull();
    expect(document.querySelector("#canvas svg")).toBeNull();
    expect(globalThis.location.search).toBe("?step=3");
  });

  it("replaces the page loaded before, its root and style with it, each id the new page's alone", () => {
    loadPage("");

    const styles = document.head.childElementCount;

    loadPage("");

    expect([document.body.childElementCount, document.head.childElementCount]).toEqual([1, styles]);
  });
});
