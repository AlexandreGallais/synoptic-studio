/** The names of an installed face, as the browser lists them. */
type InstalledFontNames = {
  /** Its family, as CSS's `font-family` names it. */
  readonly family: string;
  /** Its full name, such as `Arial Bold`. */
  readonly fullName: string;
};

/** The file of an installed face, which the browser reads on demand. */
type InstalledFontFile = {
  /** Reads the bytes of its file. */
  readonly blob: () => Promise<Blob>;
};

/** A font installed on the computer as the browser lists it, a `FontData` (`REF-WICG-LOCAL-FONT-ACCESS`): its names and its file. */
export type InstalledFont = InstalledFontNames & InstalledFontFile;
