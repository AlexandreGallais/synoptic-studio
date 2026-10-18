import type { Text } from "../text";

/**
 * The colors of a text's underline and strike-through until the designer chooses one (US-041):
 * none chosen, so that each follows the text's color and opacity and every text of F05 and F14
 * draws as before; the color kept for each, read only once chosen, black and opaque (agent's
 * reading, provisional, to confirm at VAL-016).
 */
export const TEXT_DECORATION_DEFAULTS: Pick<
  Text,
  | "hasStrikeThroughColor"
  | "hasUnderlineColor"
  | "strikeThroughColor"
  | "strikeThroughOpacity"
  | "underlineColor"
  | "underlineOpacity"
> = {
  hasStrikeThroughColor: false,
  hasUnderlineColor: false,
  strikeThroughColor: "#000000",
  strikeThroughOpacity: 100,
  underlineColor: "#000000",
  underlineOpacity: 100,
};
