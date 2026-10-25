/** The settings `saveKindSettings` keeps of the last shape of a kind. */
export type KindMemory = {
  /** The radius of each vertex kept on the radius field (`data-radii`), empty when one is typed. */
  readonly radii: string;
  /** Each input of the kind's settings, in the order the page holds them. */
  readonly settings: readonly {
    /** Whether a check box or a radio button is ticked. */
    readonly checked: boolean;
    /** Its value. */
    readonly value: string;
  }[];
};
