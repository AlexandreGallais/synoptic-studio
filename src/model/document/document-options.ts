/** The options of a document, a symbol (a synoptic view with E08), that change how it is written (ADR-0053). */
export type DocumentOptions = {
  /** Whether the faces its texts are measured with are embedded, each once, as `@font-face` rules. */
  readonly hasEmbeddedFonts: boolean;
  /** Whether each text is written with its HTML layer, a `switch` whose `foreignObject` a browser lays out (EN-041). */
  readonly hasHtmlLayer: boolean;
};
