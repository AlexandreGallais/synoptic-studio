import { textFontMetrics } from "editor";

import { readFontSession } from "../fonts";

import { readCachedValue } from "./read-cached-value";
import { TEXT_METRICS_CACHE } from "./text-metrics-cache";

import type { FontMetrics, Text } from "editor";

/**
 * Reads the metrics a text is measured with (`textFontMetrics`), kept while the values they
 * depend on stay the same: its font, fallback, weight and style, and the session's families
 * kept; a size, an opacity or a box changed reads them again from the cache.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text read from the fields
 * @returns the metrics of its face; undefined for a font without metrics nor fallback
 */
export function readTextMetrics(document: Document, text: Text): FontMetrics | undefined {
  const { kept } = readFontSession(document);

  const key = JSON.stringify([text.font, text.fallback, text.weight, text.isItalic]);

  return readCachedValue(TEXT_METRICS_CACHE, { key, source: kept }, () =>
    textFontMetrics(text, kept),
  );
}
