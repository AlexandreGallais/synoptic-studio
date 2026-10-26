import type { Text } from "editor";

/**
 * The Text tool's fields: every field of a text but its fallback, which the playground keeps at
 * `sans-serif`; X, Y and the angle go into the placement fields.
 */
export type TextFields = Omit<Text, "fallback">;
