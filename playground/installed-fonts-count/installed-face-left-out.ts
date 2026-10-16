import type { UnlistedFaceReason } from "../../src";

/** Why an installed face is left out of the font list (US-056): its file unreadable, its `fsType` forbidding embedding, or a letter of the ISO basic Latin alphabet missing. */
export type InstalledFaceLeftOut = UnlistedFaceReason | "unreadable";
