import type { PanelValues } from "../../src";

/** A field of the shape the panel edits, alone (US-085): the value it sets, then its id. */
export type EditedField = readonly [keyof PanelValues, string];
