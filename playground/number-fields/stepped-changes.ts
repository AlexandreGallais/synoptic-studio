/**
 * The `change` each arrow sends a field of its own once its step is shown within the field's
 * range (`applySteppedValue`), so that its own listeners read it as Enter does: the value it
 * shows is applied already, which the listener of a value typed leaves alone (`applyTypedValue`,
 * US-088), where reading it again as typed made a negative one relative, Up at −15 showing −14,
 * then −15 − 14 = −29.
 */
export const STEPPED_CHANGES = new WeakSet<Event>();
