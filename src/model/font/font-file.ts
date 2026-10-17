/** A font file the editor read, kept with its face so that it can be embedded (ADR-0053). */
export type FontFile = {
  /** The file's bytes, as read. */
  readonly bytes: Readonly<Uint8Array>;
  /** Whether its `fsType` permits embedding the whole file (`isEmbeddable`, `DERIV-font-embedding`). */
  readonly isEmbeddable: boolean;
  /** Its version string, name ID 5 of its `name` table, such as `Version 5.01x`; empty when it names none (`REF-OT-NAME`). */
  readonly version: string;
};
