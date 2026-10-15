/** A font file the editor read, kept with its face so that it can be embedded (ADR-0053). */
export type FontFile = {
  /** The file's bytes, as read. */
  readonly bytes: Readonly<Uint8Array>;
  /** Whether its `fsType` permits embedding the whole file (`isEmbeddable`, `DERIV-font-embedding`). */
  readonly isEmbeddable: boolean;
};
