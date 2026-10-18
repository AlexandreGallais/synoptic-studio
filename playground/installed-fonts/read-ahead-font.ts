import type { InstalledFont } from "../fonts";

/** An installed face whose family was read ahead (US-053): its metrics kept, its file not held, read again when its family is chosen. */
export type ReadAheadFont = {
  /** The face, as the browser lists it. */
  readonly font: InstalledFont;
  /** The family its family's files name, under which its metrics are kept. */
  readonly family: string;
};
