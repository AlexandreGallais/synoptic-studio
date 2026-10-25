import type { InstalledFaceLeftOut } from "./installed-face-left-out";

/** A family left out of the font list while the installed fonts are read ahead, and why. */
export type LeftOutFamily = {
  /** Its name, as the browser lists it. */
  readonly family: string;
  /** Why it is left out: that of its faces, counted once. */
  readonly reason: InstalledFaceLeftOut;
};
