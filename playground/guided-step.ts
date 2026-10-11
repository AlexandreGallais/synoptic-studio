/** One step of the guided test of the playground (VAL-001, VAL-002): what to try and what to look at. */
export type GuidedStep = {
  /** Short name of the step. */
  readonly title: string;
  /** Values the step types into the inputs. */
  readonly values: {
    /** Anchor chosen, horizontal then vertical alignment (`mid max`), used by the polygon only. */
    readonly anchor: string;
    /** Number of corners typed, used by the polygon only. */
    readonly corners: number;
    /** Height typed. */
    readonly height: number;
    /** Radius typed. */
    readonly radius: number;
    /** Shape chosen: `rectangle` or `polygon`. */
    readonly shape: string;
    /** Width typed. */
    readonly width: number;
  };
  /** What happens, in plain words. */
  readonly explanation: string;
  /** What to check on the drawing and in the text. */
  readonly look: string;
};
