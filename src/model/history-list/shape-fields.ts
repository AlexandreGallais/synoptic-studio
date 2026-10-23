/**
 * The fields of a shape a step's name is read from (US-077): each a value of the shape, compared
 * by its JSON text.
 *
 * @template T - type of the shapes, or of what is kept of each
 */
export type ShapeFields<T> = (shape: T) => Readonly<Record<string, unknown>>;
