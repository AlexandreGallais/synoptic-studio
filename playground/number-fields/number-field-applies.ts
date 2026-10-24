import type { TypedPanelValue } from "../../src";

/**
 * What each field acting as a number field whose value sets the shapes does with a value applied
 * in it, by its id, for each page document (US-085, AUD-046): one shape's fields and several's,
 * each given by `listenToNumberField`; a field it does not name holds its own value
 * (`OWN_NUMBER_FIELDS`). One mechanism reads every field's value and arrows
 * (`listenToNumberFields`), only this applying differing.
 */
export const NUMBER_FIELD_APPLIES = new WeakMap<
  Document,
  ReadonlyMap<string, (typed: TypedPanelValue | undefined) => void>
>();
