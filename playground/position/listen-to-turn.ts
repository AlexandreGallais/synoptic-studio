import { DEGREES_PER_QUARTER_TURN, degreesInTurn } from "../../src";
import { readNumber, selectElement, writeInputValue } from "../page";

import { readAngle } from "./read-angle";

/** A button turning the shape. */
type TurnButton = {
  /** Id of the button. */
  readonly id: string;
  /** Degrees it turns, clockwise when positive. */
  readonly turn: number;
};

/**
 * Reads the angle in the field, kept from 0 to 359; 0 when the field holds no integer.
 *
 * @kind procedure
 * @param document - page document
 * @returns the angle kept from 0 to 359
 * @see US-018
 */
function readShownAngle(document: Document): number {
  return Number.isSafeInteger(readNumber(document, "angle")) ? readAngle(document) : 0;
}

/**
 * Makes a button turn the shape by its angle, from the angle shown.
 *
 * @kind procedure
 * @param document - page document
 * @param button - id of the button and the degrees it turns, clockwise when positive
 * @param update - redraws the page
 * @see US-018
 */
function listenToTurnButton(document: Document, button: TurnButton, update: () => void): void {
  selectElement(document, button.id).addEventListener("click", () => {
    writeInputValue(document, "angle", degreesInTurn(readShownAngle(document) + button.turn));
    update();
  });
}

/**
 * Makes the angle field and the −90° and +90° buttons turn the shape (F04): the shape turns once
 * the angle is applied, Enter or the field left (US-087), the field then showing the angle kept
 * from 0 to 359; a button turns by a quarter turn.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see US-018
 */
export function listenToTurn(document: Document, update: () => void): void {
  const angle = selectElement(document, "angle");

  angle.addEventListener("change", () => {
    if (Number.isSafeInteger(readNumber(document, "angle"))) {
      writeInputValue(document, "angle", readAngle(document));
    }

    update();
  });

  listenToTurnButton(document, { id: "turn-left", turn: -DEGREES_PER_QUARTER_TURN }, update);
  listenToTurnButton(document, { id: "turn-right", turn: DEGREES_PER_QUARTER_TURN }, update);
}
