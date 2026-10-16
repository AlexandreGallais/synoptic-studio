import type { InstalledFaceLeftOut } from "./installed-face-left-out";
import type { LeftOutFamily } from "./left-out-family";
import type { ReadAheadCount } from "./read-ahead-count";

/** How the reading line names each reason a family is left out, in the order it says them (US-056). */
const REASONS: readonly (readonly [InstalledFaceLeftOut, string])[] = [
  ["not-embeddable", "not embeddable"],
  ["no-latin-letters", "no Latin letters"],
  ["unreadable", "unreadable"],
];

/**
 * Reads the part of the reading line that says the families left out for one reason, such as
 * `1 no Latin letters (Apple Color Emoji)`.
 *
 * @kind procedure
 * @param leftOut - the families left out
 * @param reason - the reason and how it is said
 * @returns the part; none when no family is left out for that reason
 * @see docs/domain/shapes.md#text
 */
function readReasonPart(
  leftOut: readonly LeftOutFamily[],
  reason: readonly [InstalledFaceLeftOut, string],
): readonly string[] {
  const [key, said] = reason;
  const families = leftOut.filter((family) => family.reason === key).map((left) => left.family);

  return families.length === 0
    ? []
    : [`${String(families.length)} ${said} (${families.join(", ")})`];
}

/**
 * Reads what the page says while the installed fonts are read, then once they are: the families
 * read, and those left out counted by reason — not embeddable, no Latin letters, unreadable
 * (US-056).
 *
 * @kind procedure
 * @param count - what was counted
 * @param isDone - whether every family is done
 * @returns such as `Reading the installed fonts: 12 of 312 families…`
 * @see docs/domain/shapes.md#text
 */
export function readProgressLine(count: ReadAheadCount, isDone: boolean): string {
  if (!isDone) {
    return `Reading the installed fonts: ${String(count.done)} of ${String(count.total)} families…`;
  }

  const read = `Installed fonts read: ${String(count.read)} families`;
  const parts = REASONS.flatMap((reason) => readReasonPart(count.leftOut, reason));

  return count.leftOut.length === 0
    ? `${read}.`
    : `${read}; ${String(count.leftOut.length)} left out: ${parts.join("; ")}.`;
}
