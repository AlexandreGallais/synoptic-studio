import type { InstalledFaceLeftOut } from "./installed-face-left-out";

/** What reading a family ahead gave: read, or left out and why. */
export type FamilyOutcome = InstalledFaceLeftOut | "read";
