import type { ArrowBurst } from "./arrow-burst";

/**
 * The bursts of arrows open in a field of the panel, by their document (US-081): from an arrow
 * pressed in the field to the end of the pause after its release, within which another arrow
 * joins the burst; the step of the burst is taken at the pause's end.
 */
export const ARROW_BURSTS = new WeakMap<Document, ArrowBurst>();
