import { arcHandleLength } from "../curve";

/**
 * Length of each handle of a quarter circle of radius 1 drawn as a cubic: 4(√2 − 1)/3, about
 * 0.55228 (`DERIV-round-shape-curve` step 1).
 */
export const QUARTER_CIRCLE_HANDLE = arcHandleLength(Math.PI / 2);
