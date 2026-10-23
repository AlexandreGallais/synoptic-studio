import type { PanelValues } from "./panel-values";

/**
 * Whether a field of several shapes holds a size, W or H, rather than a position, X or Y (F38).
 *
 * @kind domain
 * @param key - the field
 * @returns `true` for `width` and `height`
 * @see docs/domain/interaction.md#_2-selector
 */
export function isPanelSize(key: keyof PanelValues): key is "height" | "width" {
  return key === "width" || key === "height";
}
