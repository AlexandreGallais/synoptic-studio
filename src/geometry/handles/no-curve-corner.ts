import { ZERO_VECTOR } from "../../math";

import { NO_HANDLES } from "./no-handles";

import type { CurveCorner } from "./curve-corner";

/** A corner read where none is: at the origin, radius 0, its handles off — the fallback of a cyclic read of an empty contour. */
export const NO_CURVE_CORNER: CurveCorner = { handles: NO_HANDLES, point: ZERO_VECTOR, radius: 0 };
