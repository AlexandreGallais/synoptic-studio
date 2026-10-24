// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { PAGE_HISTORIES } from "../history";
import { readKeptDrawing } from "../kept-shapes";
import {
  clickElement,
  openServedPage,
  readDisabledState,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeInField,
} from "../user-actions";

import { HISTORY_BOUND_KEY } from "./history-bound-key";
import { readPageHistoryBound } from "./read-page-history-bound";

/**
 * Nudges the selection by Right with the focus outside the fields: one key pressed and released,
 * one step.
 *
 * @param count - how many nudges
 */
function nudge(count: number): void {
  const body = selectPageElement("body");

  const nudges = Array.from({ length: count }, () => ({
    bubbles: true,
    cancelable: true,
    key: "ArrowRight",
  }));

  for (const init of nudges) {
    body.dispatchEvent(new KeyboardEvent("keydown", init));
    body.dispatchEvent(new KeyboardEvent("keyup", init));
  }
}

/**
 * Reads how many steps the page's history holds to undo and to redo.
 *
 * @returns the past states and the states to come
 */
function readStepCounts(): { readonly past: number; readonly future: number } {
  const { future, past } = PAGE_HISTORIES.get(document) ?? { future: [], past: [] };

  return { future: future.length, past: past.length };
}

/**
 * Opens the page, the drawing given its box, so that the keys and the buttons reach it.
 */
function openBoundedPage(): void {
  openServedPage("");
  setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
}

beforeEach(() => {
  localStorage.clear();
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("the history's memory size and steps kept, set by the designer (US-076)", () => {
  it("[F40.AC4] opens on 100 MB and 5 steps kept, about 80,380 moves, the fields explained", () => {
    openBoundedPage();

    // 100 000 000 / 1 244 = 80 385.9, rounded down to tens.
    expect(readValue("history-memory")).toBe("100");
    expect(readValue("history-steps")).toBe("5");
    expect(readText("#history-size")).toBe("History 0 B of 100 MB");
    expect(readText("#history-moves")).toBe("about 80,380 moves");

    expect(selectPageElement("#history-memory").closest("label")?.title).toContain(
      "a rectangle 100 × 50 alone adds about 1,244 bytes, of the 120 × 80 the page opens with 1,248",
    );

    expect(selectPageElement("#history-steps").closest("label")?.title).toContain("whatever");

    expect(selectPageElement("#history-moves").title).toContain(
      "moves of a rectangle 100 × 50 alone the memory holds, each adding about 1,244 bytes",
    );
  });

  it("[F40.AC4] keeps 20 MB and 3 steps in the browser for the next session", () => {
    openBoundedPage();
    typeAndEnter("history-memory", "20");
    typeAndEnter("history-steps", "3");

    expect(JSON.parse(localStorage.getItem(HISTORY_BOUND_KEY) ?? "null")).toEqual({
      size: 20_000_000,
      steps: 3,
    });

    openBoundedPage();

    // 20 000 000 / 1 244 = 16 077.2.
    expect(readValue("history-memory")).toBe("20");
    expect(readValue("history-steps")).toBe("3");
    expect(readText("#history-size")).toBe("History 0 B of 20 MB");
    expect(readText("#history-moves")).toBe("about 16,070 moves");
  });

  it("[F40.AC4] bounds each step by the bound kept: 0 MB and 2 steps keep 2 of 4 nudges", () => {
    localStorage.setItem(HISTORY_BOUND_KEY, JSON.stringify({ size: 0, steps: 2 }));
    openBoundedPage();
    nudge(4);

    // The page's rectangle, 616 characters, its reference and its index: 1 248 bytes a step;
    // 2 × 1 248 = 2 496 bytes, 2.5 kB.
    expect(readStepCounts()).toEqual({ future: 0, past: 2 });
    expect(readText("#history-size")).toBe("History 2.5 kB of 0 B");
    expect(readText("#history-moves")).toBe("about 0 moves");

    clickElement("history-undo");
    clickElement("history-undo");

    expect(readDisabledState("history-undo")).toBe("true");
    // From (4, 0), two steps back: (2, 0), the first two nudges dropped.
    expect(readKeptDrawing(document)[0]?.shape).toMatchObject({ x: 2, y: 0 });
  });

  it("[F40.AC4] drops the oldest steps at once when the bound is set, never the steps to redo", () => {
    openBoundedPage();
    nudge(4);
    clickElement("history-undo");
    clickElement("history-undo");

    expect(readStepCounts()).toEqual({ future: 2, past: 2 });

    typeAndEnter("history-memory", "0");

    // 5 steps kept, 2 past: nothing dropped.
    expect(readStepCounts()).toEqual({ future: 2, past: 2 });

    typeAndEnter("history-steps", "1");

    // 1 past, 2 to come: 3 × 1 248 = 3 744 bytes, 3.7 kB.
    expect(readStepCounts()).toEqual({ future: 2, past: 1 });
    expect(readText("#history-size")).toBe("History 3.7 kB of 0 B");
  });

  it("[F40.AC4] takes no step of the drawing when a bound is applied", () => {
    openBoundedPage();
    typeAndEnter("history-memory", "50");

    expect(readStepCounts()).toEqual({ future: 0, past: 0 });
    expect(readText("#history-size")).toBe("History 0 B of 50 MB");
  });

  it.each([
    // 1 001 MB taken at 1 000; 100 − 1; 2.5 MB rounded to 3, a tie up.
    { bound: { size: 1e9, steps: 5 }, id: "history-memory", typed: "1001", want: "1000" },
    { bound: { size: 99e6, steps: 5 }, id: "history-memory", typed: "-1", want: "99" },
    { bound: { size: 3e6, steps: 5 }, id: "history-memory", typed: "2.5", want: "3" },
    // 101 steps taken at 100; 5 − 1; 1.5 rounded to 2.
    { bound: { size: 1e8, steps: 100 }, id: "history-steps", typed: "101", want: "100" },
    { bound: { size: 1e8, steps: 4 }, id: "history-steps", typed: "-1", want: "4" },
    { bound: { size: 1e8, steps: 2 }, id: "history-steps", typed: "1.5", want: "2" },
  ])("[F44.AC4] takes $typed in $id as $want, within its range", ({ bound, id, typed, want }) => {
    openBoundedPage();
    typeInField(id, typed);

    expect(readValue(id)).toBe(want);
    expect(JSON.parse(localStorage.getItem(HISTORY_BOUND_KEY) ?? "null")).toEqual(bound);
  });

  it.each([
    { id: "history-memory", typed: "", want: "100" },
    { id: "history-memory", typed: "abc", want: "100" },
    { id: "history-steps", typed: "", want: "5" },
    { id: "history-steps", typed: "1.2.3", want: "5" },
  ])("[F40.AC4] gives back $want in $id for $typed, the bound unchanged", ({ id, typed, want }) => {
    openBoundedPage();
    typeInField(id, typed);

    expect(readValue(id)).toBe(want);
    expect(localStorage.getItem(HISTORY_BOUND_KEY)).toBeNull();
    expect(readText("#history-size")).toBe("History 0 B of 100 MB");
  });

  it.each([
    { title: "the bounds themselves", typed: ["1000", "100"], want: { size: 1e9, steps: 100 } },
    { title: "none at all", typed: ["0", "0"], want: { size: 0, steps: 0 } },
  ])("[F40.AC4] takes $title", ({ typed, want }) => {
    const [memory = "", steps = ""] = typed;

    openBoundedPage();
    typeAndEnter("history-memory", memory);
    typeAndEnter("history-steps", steps);

    expect(JSON.parse(localStorage.getItem(HISTORY_BOUND_KEY) ?? "null")).toEqual(want);
  });

  it.each([
    { title: "a broken text", written: "{not a bound" },
    { title: "another value", written: '"100"' },
    { title: "sizes out of range", written: '{"size":2000000000,"steps":-1}' },
    { title: "sizes not whole", written: '{"size":1500000,"steps":2.5}' },
    { title: "texts", written: '{"size":"5","steps":"2"}' },
  ])("[F40.AC4] opens on 100 MB and 5 steps kept with $title in the browser", ({ written }) => {
    localStorage.setItem(HISTORY_BOUND_KEY, written);
    openBoundedPage();

    expect(readValue("history-memory")).toBe("100");
    expect(readValue("history-steps")).toBe("5");
    expect(readText("#history-size")).toBe("History 0 B of 100 MB");
  });

  it("[F40.AC4] keeps one value the browser wrote, the other back to its default", () => {
    localStorage.setItem(HISTORY_BOUND_KEY, '{"size":30000000,"steps":"2"}');
    openBoundedPage();

    expect(readValue("history-memory")).toBe("30");
    expect(readValue("history-steps")).toBe("5");
  });

  it("[F40.AC4] works without the browser's storage, the bound kept for the session only", () => {
    vi.spyOn(globalThis, "localStorage", "get").mockImplementation(() => {
      throw new Error("Storage blocked");
    });

    openBoundedPage();
    typeAndEnter("history-steps", "1");
    nudge(2);

    expect(readValue("history-memory")).toBe("100");
    expect(readStepCounts()).toEqual({ future: 0, past: 2 });
  });

  it("[F40.AC4] goes on with a storage full, the bound applied", () => {
    const full = new Proxy(localStorage, {
      get: (target, property): unknown =>
        property === "setItem"
          ? (): never => {
              throw new Error("QuotaExceededError");
            }
          : Reflect.get(target, property),
    });

    vi.spyOn(globalThis, "localStorage", "get").mockReturnValue(full);
    openBoundedPage();
    typeAndEnter("history-memory", "0");
    typeAndEnter("history-steps", "1");
    nudge(3);

    expect(readStepCounts()).toEqual({ future: 0, past: 1 });
  });

  it("[F40.AC4] keeps the bound at Reset, the history emptied", () => {
    openBoundedPage();
    typeAndEnter("history-memory", "7");
    nudge(1);
    clickElement("tool-reset");

    expect(readValue("history-memory")).toBe("7");
    expect(readText("#history-size")).toBe("History 0 B of 7 MB");
  });

  it("[F40.AC4] reads the default bound for a page not mounted", () => {
    const other = document.implementation.createHTMLDocument();

    expect(readPageHistoryBound(other)).toEqual({ size: 100_000_000, steps: 5 });
  });

  it("[F40.AC4] leaves the bound's fields unmarked while the shape is refused", () => {
    openBoundedPage();
    typeAndEnter("angle", "");

    expect(selectPageElement("#width").getAttribute("aria-invalid")).toBe("true");
    expect(selectPageElement("#history-memory").getAttribute("aria-invalid")).toBeNull();
    expect(selectPageElement("#history-steps").getAttribute("aria-invalid")).toBeNull();
  });

  it("[F40.AC4] leaves Ctrl + Z to a bound's field typed in, and Delete never deletes a shape", () => {
    openBoundedPage();
    nudge(1);

    const field = selectPageElement("#history-memory");
    const init = { bubbles: true, cancelable: true, ctrlKey: true, key: "z" };

    field.dispatchEvent(new Event("input", { bubbles: true }));
    field.dispatchEvent(new KeyboardEvent("keydown", init));
    field.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key: "Delete" }));

    expect(readStepCounts()).toEqual({ future: 0, past: 1 });
    expect(readKeptDrawing(document)).toHaveLength(1);
  });
});
