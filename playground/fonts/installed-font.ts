/** A font installed on the computer as the browser lists it, a `FontData` (`REF-WICG-LOCAL-FONT-ACCESS`). */
// eslint-disable-next-line functional/no-mixed-types -- the browser's FontData holds its names and its blob() method together
export type InstalledFont = {
  /** Its family, as CSS's `font-family` names it. */
  readonly family: string;
  /** Its full name, such as `Arial Bold`. */
  readonly fullName: string;
  /** Reads the bytes of its file. */
  readonly blob: () => Promise<Blob>;
};
