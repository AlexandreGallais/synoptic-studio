import { selectPageElement } from "./select-page-element";

/**
 * Whether a control is greyed, its `disabled` attribute, as a word.
 *
 * @kind procedure
 * @param id - id of the control, without `#`
 * @returns `true` or `false`
 * @see docs/backlog/stories/E14-F13-EN-030-second-pass.md
 */
export function readDisabledState(id: string): string {
  return String(selectPageElement(`#${id}`).hasAttribute("disabled"));
}
