import type { InstalledFaceLeftOut } from "./installed-face-left-out";

/** A face refused at the first reading of a family that is listed, and why. */
export type RefusedFace = {
  /** Its full name, as the browser lists it. */
  readonly name: string;
  /** Why it is refused: its file unreadable, not embeddable, or a Latin letter missing. */
  readonly reason: InstalledFaceLeftOut;
};
