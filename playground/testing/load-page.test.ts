// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { loadPage } from "./load-page";

describe("loadPage", () => {
  it("puts the page in the document without its script tag, at the address asked", () => {
    // The test mounts the playground itself: the page's own script must not be there to run it.
    loadPage("?step=3");

    expect(document.querySelector("script")).toBeNull();
    expect(document.querySelector("#canvas")).not.toBeNull();
    expect(globalThis.location.search).toBe("?step=3");
  });
});
