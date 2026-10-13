import { HANDLE_WAYS } from "../../src";
import { selectElement } from "../page";

import type { HandleWay } from "../../src";

/**
 * How the handle fields are typed, read from the list (Q32): x and y on the page, or an angle and
 * a length from the side.
 *
 * @kind procedure
 * @param document - page document
 * @returns the way chosen, `page` for a value the list does not hold
 * @see docs/backlog/stories/E14-F12-US-024-handles-as-angle-and-length.md
 */
export function readTypingWay(document: Document): HandleWay {
  const list = selectElement(document, "handle-method");

  return (
    HANDLE_WAYS.find((way) => list instanceof HTMLSelectElement && way === list.value) ?? "page"
  );
}
