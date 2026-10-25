import { selectPageElement } from "./select-page-element";

/** Where a pointer is pressed or moved, in the window, and whether a button is held. */
export type PointerPress = {
  /** The pointer's x in the window. */
  readonly clientX: number;
  /** The pointer's y in the window. */
  readonly clientY: number;
  /** The buttons held: 1 for the main one, 0 for none. */
  readonly buttons: number;
};

/**
 * Presses or moves a pointer on an element of the page: its `pointerdown` or `pointermove`
 * event, bubbling as the browser fires it.
 *
 * @kind procedure
 * @param selector - CSS selector of the element
 * @param type - `pointerdown` or `pointermove`
 * @param press - where, and the buttons held
 * @see US-035
 */
export function pressPointer(selector: string, type: string, press: PointerPress): void {
  selectPageElement(selector).dispatchEvent(
    new PointerEvent(type, { ...press, bubbles: true, pointerId: 1 }),
  );
}
