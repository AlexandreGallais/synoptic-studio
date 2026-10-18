/**
 * The address of an element of an array laid out with a fixed stride: the base address, then
 * `stride` bytes per element before it. The record 2 of an array of 16-byte table records that
 * starts at byte 12 sits at byte 44; the address of the element just past the last, at index
 * `count`, is where the array ends.
 *
 * Formula: B + c · i.
 *
 * @kind math
 * @param base - the address of element 0
 * @param stride - the size of one element, the address increment
 * @param index - which element, counted from 0
 * @returns where the element starts
 * @see REF-WIKI-ARRAY
 */
export function elementAddress(base: number, stride: number, index: number): number {
  return base + stride * index;
}
