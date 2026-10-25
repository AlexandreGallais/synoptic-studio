import { ARROW_BURSTS } from "./arrow-bursts";

/**
 * Reads the field of the burst of arrows open on the page (`ARROW_BURSTS`, US-081), whose values
 * applied by its arrows take no step, the burst taking it at its end.
 *
 * @kind procedure
 * @param document - page document
 * @returns the field; undefined with no burst open
 * @see RULE-interaction-7-navigation-and-history
 */
export function readBurstField(document: Document): Element | undefined {
  return ARROW_BURSTS.get(document)?.field;
}
