/** A burst of arrows open in a field of the panel: the field, and the pause it waits in. */
export type ArrowBurst = {
  /** The field the burst's arrows step. */
  readonly field: Element;
  /** The timer of the pause after an arrow's release; undefined while an arrow is held. */
  readonly pause: ReturnType<typeof setTimeout> | undefined;
};
