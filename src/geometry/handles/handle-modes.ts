/**
 * The ways a point's two handles follow each other: Figma's three mirroring settings
 * (`REF-FIGMA-HANDLE-MIRRORING`), and handles computed from the neighbours by a tension, as
 * Inkscape's auto-smooth nodes (`auto`, `REF-INKSCAPE-NODE-TYPES`).
 */
export const HANDLE_MODES = ["none", "angle", "angle-and-length", "auto"] as const;
