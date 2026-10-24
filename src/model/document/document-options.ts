/** The options of a document, a symbol (a synoptic view with E38), that change how it is written (ADR-0053, ADR-0060). */
export type DocumentOptions = {
  /** Whether the faces its texts are measured with are embedded, each once, as `@font-face` rules, in a file exported or saved (ADR-0060). */
  readonly hasEmbeddedFonts: boolean;
};
