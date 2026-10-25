import { nearestInteger, percentFromFraction } from "../../src";
import { writeInputValue, writeText } from "../page";
import { readView } from "../view";

/**
 * Shows the zoom in the toolbar (SP-030's answer 9): its percentage, rounded to the nearest integer
 * (`percentFromFraction`, `nearestInteger`), on the menu's button and in its field; × √2 shows 141 %, ÷ √2 71 %.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-7-navigation-and-history
 */
export function showZoom(document: Document): void {
  const percentage = nearestInteger(percentFromFraction(readView(document).scale));

  writeText(document, "zoom-percentage", `${String(percentage)} %`);
  writeInputValue(document, "zoom-field", percentage);
}
