/** How an `@font-face` rule names a font file's format: its media type in the `data:` URL and its format hint (`REF-CSS-FONTS-4`, §11.2). */
export type FontFormat = {
  /** The media type: `font/ttf`, or `font/otf` for a file with CFF outlines. */
  readonly mediaType: string;
  /** The hint of `format()`: `truetype`, or `opentype` for a file with CFF outlines. */
  readonly hint: string;
};
