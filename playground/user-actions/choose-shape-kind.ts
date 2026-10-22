import { chooseOption } from "./choose-option";

/**
 * Sets the kind of the shape the panel edits through the page's kind selector, as its `change`
 * event does, without pressing a tool button: a test that needs a polygon or a text stays true
 * whatever the tool buttons do (EN-142). The X, Y and angle are the shape's, as they were.
 *
 * @kind procedure
 * @param kind - `rectangle`, `polygon`, `round` or `text`
 * @see docs/backlog/stories/E02-F35-EN-142-playground-tests-without-tool-buttons.md
 */
export function chooseShapeKind(kind: string): void {
  chooseOption("shape", kind);
}
