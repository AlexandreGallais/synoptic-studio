import type { Text } from "../../../src";

/**
 * The Text tool's fields a step of F05 or F14 types: every field of a text but
 * its fallback, which the playground keeps at `sans-serif`; X, Y and the angle go into the
 * placement fields.
 */
export type GuidedText = Omit<Text, "fallback">;
