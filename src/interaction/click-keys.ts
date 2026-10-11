/** Modifier keys held during a click on a vertex (`docs/domain/interaction.md` §3). */
export type ClickKeys = {
  /** Shift held. */
  readonly shift: boolean;
  /** Ctrl held, or Cmd on macOS. */
  readonly toggle: boolean;
};
