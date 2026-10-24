/**
 * The fields of the panel typed in since their value was last applied, by Enter or by leaving
 * them (F40): such a field keeps Ctrl + Z, Ctrl + Shift + Z and Ctrl + Y, any other leaves them to
 * the drawing (SP-029's answer 9).
 */
export const TYPED_FIELDS = new WeakSet<Element>();
