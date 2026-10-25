import type { TypedPanelValue } from "./typed-panel-value";

/**
 * A value once a value is typed in its field: its own plus the value typed with a
 * sign, or the value typed without one, as Figma (`REF-FIGMA-ADJUST`); 100 and `+50` give 150,
 * 30 and `-5` give 25, `460` gives 460, a fraction kept.
 *
 * @kind domain
 * @param own - the value the field showed, or a shape's own in it
 * @param typed - the value typed, or an arrow's step, relative
 * @returns the value, as computed from the numbers typed
 * @see RULE-interaction-2-selector
 */
export function valueWithTyped(own: number, typed: TypedPanelValue): number {
  return typed.isRelative ? own + typed.value : typed.value;
}
