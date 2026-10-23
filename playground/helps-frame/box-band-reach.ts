/** How far a band about a box's sides reaches on each side of them, in user units. */
export type BoxBandReach = {
  /** How far it reaches outside the sides. */
  readonly outward: number;
  /** How far it reaches inside the sides. */
  readonly inward: number;
};
