import type { PanelValues } from "editor";

/** A field of the shape the panel edits, alone: the value it sets, then its id. */
export type EditedField = readonly [keyof PanelValues, string];
