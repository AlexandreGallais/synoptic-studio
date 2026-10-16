import type { InstalledFaceLeftOut } from "./installed-face-left-out";

/** What reading a family ahead gave: read, left out and why, or already read because it was chosen meanwhile. */
export type FamilyOutcome = InstalledFaceLeftOut | "chosen" | "read";
