import { listenToInputs } from "./listen-to-inputs";
import { readGuidedStep } from "./read-guided-step";
import { readShape } from "./read-shape";
import { readStepFromAddress } from "./read-step-from-address";
import { selectElement } from "./select-element";
import { showGuidedStep } from "./show-guided-step";
import { showRefusedShape } from "./show-refused-shape";
import { showShape } from "./show-shape";
import { writeCappedSizes } from "./write-capped-sizes";
import { writeValidity } from "./write-validity";

/**
 * Updates the page from the inputs: the selected shape is drawn when valid, refused otherwise;
 * the number of corners and the anchor are shown for the polygon only.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function updatePlayground(document: Document): void {
  writeCappedSizes(document);

  const reading = readShape(document);
  const isPolygon = "anchor" in reading.model;

  selectElement(document, "corners-field").hidden = !isPolygon;
  selectElement(document, "anchor-field").hidden = !isPolygon;
  writeValidity(document, reading);

  if (reading.isValid) {
    showShape(document, reading);
  } else {
    showRefusedShape(document);
  }
}

/**
 * Wires the Previous and Next buttons of the guided test.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/backlog/stories/E01-F01-VAL-001-validate-rectangle.md
 */
function listenToGuide(document: Document, update: () => void): void {
  selectElement(document, "guide-previous").addEventListener("click", () => {
    showGuidedStep(document, readGuidedStep(document) - 1, update);
  });

  selectElement(document, "guide-next").addEventListener("click", () => {
    showGuidedStep(document, readGuidedStep(document) + 1, update);
  });
}

/**
 * Mounts the playground: redraws on every input and window resize, wires the guided test, then
 * shows the step asked by the page address, or the first one.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function mountPlayground(document: Document): void {
  /** Redraws the page from the inputs. */
  function update(): void {
    updatePlayground(document);
  }

  listenToInputs(document, update);

  listenToGuide(document, update);

  showGuidedStep(document, readStepFromAddress(document), update);
}
