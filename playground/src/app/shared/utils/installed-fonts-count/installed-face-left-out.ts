import type { UnlistedFaceReason } from "editor";

/** Why an installed face is left out of the font list: its file unreadable, its `fsType` forbidding embedding, or a letter of the ISO basic Latin alphabet missing. */
export type InstalledFaceLeftOut = UnlistedFaceReason | "unreadable";
