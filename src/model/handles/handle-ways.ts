/**
 * The ways to type a handle (Q32, decided at VAL-011): its end on the page (`page`), as in Figma,
 * or an angle and a length from its side (`side`), so that the same values give the same bend at
 * every point.
 */
export const HANDLE_WAYS = ["page", "side"] as const;
