import type { KeyModifiers } from "./key-modifiers";

/**
 * Tells a key pressed with Ctrl, Cmd or Alt held, which the page leaves to the browser
 * (`interaction.md` §1): such a key nudges, holds and steps no arrow, and zooms nothing, so that
 * the browser keeps the keys it gives them; on macOS a key pressed with Cmd held sends no `keyup`
 * (`REF-WEBKIT-CMD-KEYUP`, `REF-MOZILLA-CMD-KEYUP`), so an arrow held from it would never be
 * released (AUD-045).
 *
 * @kind domain
 * @param modifiers - the modifiers held with the key
 * @returns true with Ctrl, Cmd or Alt held
 * @see RULE-interaction-1-tools
 */
export function isBrowserKey(modifiers: KeyModifiers): boolean {
  return [modifiers.ctrlKey, modifiers.metaKey, modifiers.altKey].includes(true);
}
