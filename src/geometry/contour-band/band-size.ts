/** The size of a band about a contour (`contourBandWithin`): its half width and its longest chord. */
export type BandSize = {
  /** Half the band's width, in user units, > 0: half a stroke's width, the radius of its round join. */
  readonly halfWidth: number;
  /** The longest chord wanted along a curve, in user units, > 0. */
  readonly step: number;
};
