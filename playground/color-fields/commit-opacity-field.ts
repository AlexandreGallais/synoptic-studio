import { TEXT_RANGES, isInIntegerRange } from "../../src";
import { readNumber } from "../page";

import { writeColorChosen } from "./write-color-chosen";

import type { ColorField } from "./color-field";

/**
 * Commits what an opacity field holds when it is left or Enter pressed: an opacity the model
 * accepts, an integer from 0 to 100, marks a color that follows the text's as chosen
 * (`writeColorChosen`, US-041); a refused one leaves it following, the text refused until it is
 * put right.
 *
 * @kind procedure
 * @param document - page document
 * @param ids - the color's fields
 * @see docs/backlog/stories/E15-F16-US-041-underline-and-strike-through-colors.md
 */
export function commitOpacityField(document: Document, ids: ColorField): void {
  const { opacity } = TEXT_RANGES;

  if (isInIntegerRange(readNumber(document, ids.opacity), opacity.first, opacity.last)) {
    writeColorChosen(document, ids);
  }
}
