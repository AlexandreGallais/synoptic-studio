import type { View } from "../../src";

/**
 * The view the page opens with, and Reset and each step of the guided test go back to (ADR-0098,
 * SP-030's answer 6): 100 %, the drawing's point (−40, −40) at the canvas's top-left, a margin of 40
 * pixels for a point or a box a little left of or above 0, such as a stretched polygon rounded past
 * its box (REV-001, step 72); the frame of every file exported.
 */
export const OPENING_VIEW: View = { origin: { x: -40, y: -40 }, scale: 1 };
