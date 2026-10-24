import { NO_VERTEX_SELECTION } from "../../src";

import { selectElement } from "./select-element";
import { writeSelection } from "./write-selection";

/**
 * Makes the panel edit a shape of a kind, without firing any event: no free shape kept, no point
 * selected; `none` while no shape is selected (F36).
 *
 * @kind procedure
 * @param document - page document
 * @param kind - `rectangle`, `polygon`, `round`, `text` or `none`
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeEditedKind(document: Document, kind: string): void {
  const select = selectElement(document, "shape");

  delete select.dataset["freeShape"];
  select.querySelector('option[value="free"]')?.setAttribute("hidden", "");
  Object.assign(select, { value: kind });
  writeSelection(document, NO_VERTEX_SELECTION);
}
