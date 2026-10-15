import { textLineHeight, textLines } from "../text-block";
import { textInCase } from "../text-case";

import { htmlLayerFrame } from "./html-layer-frame";
import { htmlLayerLineLimit } from "./html-layer-line-limit";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { DrawnText } from "../text-drawing";
import type { HtmlLayer } from "./html-layer";

/**
 * What a text's HTML layer is written from (ADR-0053, decision 1): its rectangle
 * (`htmlLayerFrame`), the library's line height (`textLineHeight`), the most lines its box shows
 * (`htmlLayerLineLimit`), and its content written in its case and cut at each line break typed,
 * which the browser wraps itself (`textInCase`, `textLines`).
 *
 * @kind domain
 * @param metrics - the metrics of the face the text is drawn with
 * @param text - its content, case, box and options
 * @param drawn - its block and its lines, from `textDrawing`
 * @returns its layer
 * @see DERIV-html-layer
 */
export function textHtmlLayer(metrics: FontMetrics, text: Text, drawn: DrawnText): HtmlLayer {
  return {
    frame: htmlLayerFrame(metrics, text, drawn),
    lineHeight: textLineHeight(metrics, text),
    lineLimit: htmlLayerLineLimit(metrics, text),
    paragraphs: textLines(textInCase(text.content, text.textCase)),
  };
}
