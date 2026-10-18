// Reading the JSON of Claude Code's transcript lines and hook inputs (docs/tooling/metrics.md):
// properties of unknown values.
// The hooks read JSON in Rust (`tools/hooks`, EN-104); this file stays for the metrics scripts that
// import it until EN-107 ports them.

/**
 * Property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property, or `undefined` when `value` is not an object
 */
export function field(value: unknown, key: string): unknown {
  return typeof value === "object" && value !== null ? Reflect.get(value, key) : undefined;
}

/**
 * String property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a string, else an empty string
 */
export function text(value: unknown, key: string): string {
  const property = field(value, key);

  return typeof property === "string" ? property : "";
}

/**
 * String property of a parsed JSON value, or a fallback when it is missing or empty.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @param fallback - text used instead of an empty one
 * @returns the property, or `fallback`
 */
export function textOr(value: unknown, key: string, fallback: string): string {
  const found = text(value, key);

  return found === "" ? fallback : found;
}

/**
 * Number property of a parsed JSON value.
 *
 * @param value - parsed JSON
 * @param key - property name
 * @returns the property when it is a number, else 0
 */
export function count(value: unknown, key: string): number {
  const property = field(value, key);

  return typeof property === "number" ? property : 0;
}
