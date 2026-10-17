import type { Text } from "../text";

/** What a line's width depends on besides its characters and its font. */
export type LineSpacing = Pick<Text, "letterSpacing" | "size" | "wordSpacing">;
