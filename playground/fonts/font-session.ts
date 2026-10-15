import type { InstalledFont } from "./installed-font";
import type { FontFamily } from "../../src";

/** The fonts of the page while it stays open (US-036): kept for the session, not across sessions (E02). */
export type FontSession = {
  /** The families whose metrics are read: Inter first, then those imported or read from the computer, in their order. */
  readonly kept: readonly FontFamily[];
  /** The installed fonts the browser listed, their files not read yet, in its order. */
  readonly installed: readonly InstalledFont[];
};
