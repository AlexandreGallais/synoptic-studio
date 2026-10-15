/** The names of an installed face, as the browser lists them. */
type InstalledFontNames = {
  /** Its family, as CSS's `font-family` names it. */
  readonly family: string;
  /** Its full name, such as `Arial Bold`. */
  readonly fullName: string;
  /** Its PostScript name, unique to the face, such as `Arial-BoldMT`: its name ID 6. */
  readonly postscriptName: string;
};

/** The file of an installed face, which the browser reads on demand. */
type InstalledFontFile = {
  /** Reads the bytes of its file. */
  readonly blob: () => Promise<Blob>;
};

/** A font installed on the computer as the browser lists it, a `FontData` (`REF-WICG-LOCAL-FONT-ACCESS`): its names and its file. */
export type InstalledFont = InstalledFontNames & InstalledFontFile;
