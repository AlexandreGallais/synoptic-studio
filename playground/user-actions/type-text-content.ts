import { selectPageElement } from "./select-page-element";

/**
 * Types the content of the Text tool's text, then leaves its Content field, as the designer does
 * after placing a text with no content of its own (US-075): its `input`, its `change`, then its
 * `focusout`.
 *
 * @kind procedure
 * @param content - what is typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function typeTextContent(content: string): void {
  const area = selectPageElement("#text-content");

  Object.assign(area, { value: content });
  area.dispatchEvent(new Event("input", { bubbles: true }));
  area.dispatchEvent(new Event("change", { bubbles: true }));
  area.dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
}
