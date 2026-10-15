/**
 * Why a face is not embedded (ADR-0053): its file is not held (Inter, whose metrics ship without
 * their files); its `fsType` does not permit it; another face of its family used may not be
 * embedded, the family embedded whole or not at all; its family is named like a generic family;
 * or no text names its family, the face reached only as the default font.
 */
export type LeftOutReason =
  "family-incomplete" | "generic-name" | "no-file" | "not-named" | "not-permitted";
