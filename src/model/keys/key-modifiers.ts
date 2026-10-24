/** The modifiers held as a key is pressed, as a `KeyboardEvent` holds them (`interaction.md` §1). */
export type KeyModifiers = {
  /** Whether Alt, or Option on macOS, is held. */
  readonly altKey: boolean;
  /** Whether Ctrl is held. */
  readonly ctrlKey: boolean;
  /** Whether Cmd on macOS, or the Windows key, is held. */
  readonly metaKey: boolean;
};
