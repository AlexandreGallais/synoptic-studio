// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { openServedPage, selectPageElement } from "../user-actions";

describe("[F18.AC12] the checklist's panel beside the drawing (EN-056)", () => {
  it("is a column of the page right of the drawing, not a card over it", () => {
    openServedPage("?checklist=E15");

    const stage = selectPageElement("#stage");
    const checklist = selectPageElement("#checklist");
    const style = getComputedStyle(checklist);

    // A column of the page: a child of the body, after the stage holding the drawing, outside it.
    expect(checklist.parentElement).toBe(document.body);
    expect(stage.contains(checklist)).toBe(false);
    expect(stage.nextElementSibling).toBe(checklist);
    expect(getComputedStyle(document.body).display).toBe("flex");
    // In the flow, its own width, never shrunk over by the drawing nor floating above it.
    // happy-dom gives an empty position when no rule sets one: the default, static.
    expect(["", "static"]).toContain(style.position);
    expect(style.flexGrow).toBe("0");
    expect(style.flexShrink).toBe("0");
    expect(style.flexBasis).toBe("24rem");
    expect(style.display).toBe("block");
    // The drawing takes the room left, and only it.
    expect(getComputedStyle(stage).flexGrow).toBe("1");
  });

  it("leaves its room to the drawing when the address asks no checklist", () => {
    openServedPage("");

    expect(getComputedStyle(selectPageElement("#checklist")).display).toBe("none");
  });
});
