import type { FontFace } from "./font-face";

/** A weight's distance to the one asked is under 1000, the span of CSS weights (1 to 1000): each later step of the search adds it once more, so that it ranks after every face of the steps before. */
const SEARCH_STEP = 1000;

/** The lightest weight of the middle range, searched upward first. */
const MIDDLE_FIRST = 400;

/** The heaviest weight of the middle range. */
const MIDDLE_LAST = 500;

/**
 * Where a weight comes in the search for a weight under 400: the weights at or under it in
 * descending order, then those above it in ascending order.
 *
 * @kind domain
 * @param desired - the weight asked
 * @param weight - a face's weight
 * @returns its rank, the lowest searched first
 * @see REF-CSS-FONTS-4
 */
function lightRank(desired: number, weight: number): number {
  return weight <= desired ? desired - weight : SEARCH_STEP + weight - desired;
}

/**
 * Where a weight comes in the search for a weight over 500: the weights at or above it in
 * ascending order, then those under it in descending order.
 *
 * @kind domain
 * @param desired - the weight asked
 * @param weight - a face's weight
 * @returns its rank, the lowest searched first
 * @see REF-CSS-FONTS-4
 */
function heavyRank(desired: number, weight: number): number {
  return weight >= desired ? weight - desired : SEARCH_STEP + desired - weight;
}

/**
 * Where a weight comes in the search for a weight of 400 or more: for one over 500, the weights
 * at or above it in ascending order, then those under it in descending order; for one from 400 to
 * 500, the same, but the weights past 500 come last, in ascending order.
 *
 * @kind domain
 * @param desired - the weight asked
 * @param weight - a face's weight
 * @returns its rank, the lowest searched first
 * @see REF-CSS-FONTS-4
 */
function upperRank(desired: number, weight: number): number {
  return desired <= MIDDLE_LAST && weight > MIDDLE_LAST
    ? 2 * SEARCH_STEP + weight - desired
    : heavyRank(desired, weight);
}

/**
 * Where a weight comes in the search for the weight asked (CSS Fonts 4, §5.2).
 *
 * @kind domain
 * @param desired - the weight asked
 * @param weight - a face's weight
 * @returns its rank, the lowest searched first
 * @see REF-CSS-FONTS-4
 */
function weightRank(desired: number, weight: number): number {
  return desired < MIDDLE_FIRST ? lightRank(desired, weight) : upperRank(desired, weight);
}

/**
 * The face of a family that draws a text of a weight and a style, as CSS matches it: the faces
 * of the style asked (italic or upright), or of the other style when the family has none; among
 * them the weight asked, else the weight found first by the search of CSS Fonts 4, §5.2.
 *
 * @kind domain
 * @param faces - the family's faces
 * @param style - the weight and the style asked
 * @returns the matching face; undefined for a family without faces
 * @see REF-CSS-FONTS-4
 */
export function matchingFace(
  faces: readonly FontFace[],
  style: Pick<FontFace, "isItalic" | "weight">,
): FontFace | undefined {
  const ofStyle = faces.filter((face) => face.isItalic === style.isItalic);

  return (ofStyle.length > 0 ? ofStyle : faces).reduce<FontFace | undefined>(
    (best, face) =>
      best === undefined ||
      weightRank(style.weight, face.weight) < weightRank(style.weight, best.weight)
        ? face
        : best,
    undefined,
  );
}
