/**
 * The fields acting as number fields holding a text typed and not applied yet (AUD-046): marked
 * at its `input`, cleared as a value is applied (`change`) or the page writes one in it
 * (`writeInputValue`), so that a redraw meanwhile reads the value applied (`readShownText`).
 */
export const TYPED_NUMBER_FIELDS = new WeakSet<Element>();
