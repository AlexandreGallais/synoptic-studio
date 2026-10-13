// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { clickElement, openPage, readText } from "../user-actions";

describe("guided test address", () => {
  it("opens on the step the address asks, and writes the step shown into the address", () => {
    openPage("?step=20");

    expect(readText("#guide-number")).toBe("20");
    expect(readText("#guide-title")).toBe("F07 · A triangle and its box");

    clickElement("guide-next");

    expect(globalThis.location.search).toBe("?step=21");
  });

  it("opens on the first step when the address asks none, or no integer", () => {
    openPage("");

    expect(readText("#guide-number")).toBe("1");
    expect(globalThis.location.search).toBe("?step=1");

    openPage("?step=abc");

    expect(readText("#guide-number")).toBe("1");
  });

  it("opens on the last step when the address asks one beyond it", () => {
    openPage("?step=999");

    expect(readText("#guide-number")).toBe(readText("#guide-count"));
  });
});
