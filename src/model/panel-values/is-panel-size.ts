import type { PanelValues } from "./panel-values";

/**
 * Whether a field of several shapes holds a size, W or H, rather than a position, X or Y.
 *
 * @kind domain
 * @param key - the field
 * @returns `true` for `width` and `height`
 * @see RULE-interaction-2-selector
 */
export function isPanelSize(key: keyof PanelValues): key is "height" | "width" {
  return key === "width" || key === "height";
}
