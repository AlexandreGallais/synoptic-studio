/**
 * A node of the held counts (`HeldCounts`, EN-147): at the last level, how many states hold a
 * shape; above it, up to 32 nodes, one for each digit in base 32 of the shapes' keys, none for a
 * digit no shape held has (ADR-0097).
 */
export type HeldNode = number | readonly (HeldNode | undefined)[];
