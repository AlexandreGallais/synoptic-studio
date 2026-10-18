import { LEFT_OUT_REASONS } from "./left-out-reasons";

import type { InstalledFaceLeftOut } from "./installed-face-left-out";
import type { ReadAheadCount } from "./read-ahead-count";
import type { RefusedFace } from "./refused-face";

/** A family or a face the reading line names, and why it was left out or refused. */
type NamedReason = {
  /** Its name. */
  readonly name: string;
  /** Why. */
  readonly reason: InstalledFaceLeftOut;
};

/**
 * Reads the part of the reading line that says the families or faces left out for one reason,
 * such as `1 no Latin letters (Apple Color Emoji)`.
 *
 * @kind procedure
 * @param named - the families or faces left out
 * @param reason - the reason and how it is said
 * @returns the part; none when none is left out for that reason
 * @see docs/domain/shapes.md#text
 */
function readReasonPart(
  named: readonly NamedReason[],
  reason: readonly [InstalledFaceLeftOut, string],
): readonly string[] {
  const [key, said] = reason;
  const names = named.filter((left) => left.reason === key).map((left) => left.name);

  return names.length === 0 ? [] : [`${String(names.length)} ${said} (${names.join(", ")})`];
}

/**
 * Reads the parts that say families or faces left out, counted by reason, in the one order of
 * `LEFT_OUT_REASONS`.
 *
 * @kind procedure
 * @param named - the families or faces left out
 * @returns the parts, such as `1 not embeddable (Test Locked); 1 unreadable (Test Bad)`
 * @see docs/domain/shapes.md#text
 */
function readReasonParts(named: readonly NamedReason[]): string {
  return LEFT_OUT_REASONS.flatMap((reason) => readReasonPart(named, reason)).join("; ");
}

/**
 * Reads the sentence that says the faces refused in the families read, counted by reason
 * (US-062).
 *
 * @kind procedure
 * @param refused - the faces refused
 * @returns the sentence after a space, such as ` 1 faces of the families read refused: 1 unreadable (Helvetica Light).`; empty when none was refused
 * @see docs/domain/shapes.md#text
 */
function readRefusedSentence(refused: readonly RefusedFace[]): string {
  return refused.length === 0
    ? ""
    : ` ${String(refused.length)} faces of the families read refused: ${readReasonParts(refused)}.`;
}

/**
 * Reads what the page says while the installed fonts are read, then once they are: the families
 * read, those left out counted by reason — not embeddable, no Latin letters, unreadable (US-056)
 * —, then the faces refused in the families read, counted the same way (US-062).
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
  const families = count.leftOut.map((left) => ({ name: left.family, reason: left.reason }));
  const leftOut =
    families.length === 0
      ? read
      : `${read}; ${String(families.length)} left out: ${readReasonParts(families)}`;

  return `${leftOut}.${readRefusedSentence(count.refused)}`;
}
