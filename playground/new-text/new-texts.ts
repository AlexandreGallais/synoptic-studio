/**
 * The pages whose new text is being typed, by their document (F40, SP-029's answer 5): from the
 * click or the drag that placed it until its Content field is first left with a content typed, or
 * the panel leaves it; meanwhile no step is taken, so that the text placed and what is typed in it
 * are one step.
 */
export const NEW_TEXTS = new WeakSet<Document>();
