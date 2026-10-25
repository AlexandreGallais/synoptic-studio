// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { openPage, readValue, runResizeObservers, selectPageElement } from "../user-actions";

import { readListenerSignal } from "./read-listener-signal";
import { removeListeners } from "./remove-listeners";

/**
 * Opens the page as each test's `beforeEach` does, several times in the same document.
 *
 * @param count - how many mounts, >= 0
 */
function openTimes(count: number): void {
  if (!(count > 0)) {
    return;
  }

  openPage("");
  openTimes(count - 1);
}

/**
 * Counts the page's redraws from here on: each redraw asks an animation frame (`scheduleRedraw`),
 * run at once, as `loadPage` runs them.
 *
 * @returns the redraws asked so far, read when called
 */
function countRedraws(): () => number {
  const asked: FrameRequestCallback[] = [];

  Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
    configurable: true,
    value: (callback: FrameRequestCallback): number => {
      asked.push(callback);
      callback(0);

      return asked.length;
    },
  });

  return () => asked.length;
}

/**
 * Presses a key on the page outside its fields, then releases it, as the browser sends them,
 * several times.
 *
 * @param init - the key and the modifiers held
 * @param count - how many presses, >= 0
 */
function pressTimes(init: KeyboardEventInit, count: number): void {
  if (!(count > 0)) {
    return;
  }

  const body = selectPageElement("body");

  body.dispatchEvent(new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true }));
  body.dispatchEvent(new KeyboardEvent("keyup", { ...init, bubbles: true, cancelable: true }));
  pressTimes(init, count - 1);
}

describe("the listeners of a mount removed at the next one (REF-MDN-ADD-EVENT-LISTENER)", () => {
  it.each([1, 2, 48])(
    "[F43.AC1] redraws the page 14 times for 7 undos by Ctrl + Z after %i mounts, each key heard once",
    // After 48 mounts, alone: 469 ms, the median of 5 runs of its file (451 to 498 ms); over
    // Vitest's 5 s under the pre-push's load twice on 2026-10-25, so run alone after the lanes;
    // 2 s is 4.3 times it.
    { tags: ["timing"], timeout: 2000 },
    (mounts) => {
      openTimes(mounts);

      const x = Number(readValue("shape-x"));

      // The rectangle the page opens with, selected: 7 nudges by Right, one step each.
      pressTimes({ key: "ArrowRight" }, 7);
      expect(Number(readValue("shape-x"))).toBe(x + 7);

      const redraws = countRedraws();

      pressTimes({ ctrlKey: true, key: "z" }, 7);

      // Each undo redraws at its key down (`moveHistory`) and asks a step at its key up
      // (`scheduleHistoryStep`), which redraws once more: 2 × 7 = 14, against 49 per key after
      // 48 mounts when every mount's listeners stayed.
      expect(redraws()).toBe(14);
      // 7 undos for the 7 nudges: the rectangle back where it was.
      expect(Number(readValue("shape-x"))).toBe(x);
    },
  );

  it("[F43.AC1] redraws the page once for the canvas resized after 2 mounts", () => {
    openTimes(2);

    const redraws = countRedraws();

    runResizeObservers("#canvas");

    expect(redraws()).toBe(1);
  });

  it("reads the same signal for a document until its listeners are removed, then a new one", () => {
    const own = document.implementation.createHTMLDocument("");
    const signal = readListenerSignal(own);

    expect(readListenerSignal(own)).toBe(signal);
    expect(signal.aborted).toBe(false);

    removeListeners(own);

    // Aborted, the listeners given it are removed; the next mount's get a new one.
    expect(signal.aborted).toBe(true);
    expect(readListenerSignal(own)).not.toBe(signal);
    expect(readListenerSignal(own).aborted).toBe(false);
  });

  it("removes nothing from a document never mounted, the signal read after it new", () => {
    const own = document.implementation.createHTMLDocument("");

    removeListeners(own);

    expect(readListenerSignal(own).aborted).toBe(false);
  });
});
