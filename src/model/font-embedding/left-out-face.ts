import type { LeftOutReason } from "./left-out-reason";

/** A face a document's texts are measured with but that it does not embed, and why, for the editor to say so (ADR-0053). */
export type LeftOutFace = {
  /** The family's name, as read. */
  readonly family: string;
  /** Weight, from 100 (thin) to 900 (black). */
  readonly weight: number;
  /** Whether the face is italic. */
  readonly isItalic: boolean;
  /** Why it is not embedded. */
  readonly reason: LeftOutReason;
};
