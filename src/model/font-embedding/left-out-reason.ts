/** Why a face is not embedded: its file is not held (Inter, whose metrics ship without their files), or its `fsType` does not permit it (ADR-0053). */
export type LeftOutReason = "no-file" | "not-permitted";
