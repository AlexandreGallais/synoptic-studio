import { CODE_UNIT_BYTES } from "./code-unit-bytes";
import { shapeTextLength } from "./shape-text-length";

/**
 * The bytes of a shape the history holds (F40): its JSON text's length (`shapeTextLength`) times
 * 2, a string being a sequence of 16-bit code units (`CODE_UNIT_BYTES`,
 * `REF-ECMA-262-STRING-TYPE`).
 *
 * @kind domain
 * @param shape - the shape, or what is kept of it, an object never changed once made
 * @returns its bytes, twice its text's length
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function shapeBytes(shape: Readonly<WeakKey>): number {
  return shapeTextLength(shape) * CODE_UNIT_BYTES;
}
