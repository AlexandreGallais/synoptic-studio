// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import page from "./index.html?raw";
import { mountPlayground } from "./mount-playground";

/**
 * Opens the playground at an address, as a link would: the page as served, without its script
 * tag, mounted by the test.
 *
 * @param search - query of the address, e.g. `?step=20`
 */
function open(search: string): void {
  globalThis.history.replaceState(undefined, "", `/${search}`);

  const parser = new DOMParser();
  const parsed = parser.parseFromString(page, "text/html");

  parsed.querySelector("script")?.remove();
  document.body.replaceChildren(...parsed.body.childNodes);
  mountPlayground(document);
}

/**
 * Text shown in an element of the page.
 *
 * @param id - element id, without `#`
 * @returns its text content, empty when absent
 */
function text(id: string): string {
  return document.querySelector(`#${id}`)?.textContent ?? "";
}

describe("guided test address", () => {
  it("opens on the step the address asks, and writes the step shown into the address", () => {
    open("?step=20");

    expect(text("guide-number")).toBe("20");
    expect(text("guide-title")).toBe("F07 · A triangle and its box");

    document.querySelector<HTMLButtonElement>("#guide-next")?.click();

    expect(globalThis.location.search).toBe("?step=21");
  });

  it("opens on the first step when the address asks none, or no integer", () => {
    open("");

    expect(text("guide-number")).toBe("1");
    expect(globalThis.location.search).toBe("?step=1");

    open("?step=abc");

    expect(text("guide-number")).toBe("1");
  });

  it("opens on the last step when the address asks one beyond it", () => {
    open("?step=999");

    expect(text("guide-number")).toBe(text("guide-count"));
  });
});
