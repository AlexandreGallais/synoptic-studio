import type { InstalledFont } from "./installed-font";
import type { FontFamily } from "../../src";
import type { ReadAheadFont } from "../installed-fonts";

/** The fonts of the page while it stays open: kept for the session, not across sessions. */
export type FontSession = {
  /** The families whose metrics are read: Inter first, then those imported or read from the computer, in their order. */
  readonly kept: readonly FontFamily[];
  /** The installed fonts the browser listed, their files not read yet, in its order. */
  readonly installed: readonly InstalledFont[];
  /** The installed faces read ahead, their metrics kept without their files, read again when their family is chosen. */
  readonly readAhead: readonly ReadAheadFont[];
  /** The kept family the text is measured with: the one chosen in the font list, or, while an installed family chosen is read, the one chosen before it. */
  readonly measured: string;
  /** Whether the installed fonts the browser listed are being read: ahead, family after family, then the default font's files again. */
  readonly isReading: boolean;
};
