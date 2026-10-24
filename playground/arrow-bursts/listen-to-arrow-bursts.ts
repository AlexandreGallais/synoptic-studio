import { readListenerSignal } from "../listeners";
import { selectElement } from "../page";
import { readKeyField } from "../selection-keys";

import { ARROW_BURSTS } from "./arrow-bursts";

/**
 * The fields of the panel whose arrows step a value, a burst of them one step: the number fields,
 * which the browser steps, and the fields of several shapes, which the page steps.
 */
const BURST_FIELDS = 'input[type="number"], #several-fields input';

/** The arrows that step a field: Up and Down. */
const STEP_ARROWS = new Set(["ArrowDown", "ArrowUp"]);

/**
 * The pause after an arrow's release within which another arrow joins its burst, in ms: as text
 * editors group typing into one step, ProseMirror after 500 ms (`REF-PROSEMIRROR-HISTORY`) and
 * Quill within 1 000 ms (`REF-QUILL-HISTORY`), the shorter, so that two bursts a second apart
 * stay two steps.
 */
const BURST_PAUSE = 500;

/**
 * Ends the burst of arrows of the page, if one is open: its pause's timer cleared, then its step
 * taken.
 *
 * @kind procedure
 * @param document - page document
 * @param step - asks the step of the history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function stopArrowBurst(document: Document, step: () => void): void {
  const burst = ARROW_BURSTS.get(document);

  if (burst === undefined) {
    return;
  }

  clearTimeout(burst.pause);
  ARROW_BURSTS.delete(document);
  step();
}

/**
 * Reads the field whose arrows step a value a key is pressed in (`BURST_FIELDS`).
 *
 * @kind procedure
 * @param event - a key pressed in the panel
 * @returns the field; undefined outside such a field
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function readSteppedField(event: KeyboardEvent): Element | undefined {
  return readKeyField(event)?.closest(BURST_FIELDS) ?? undefined;
}

/**
 * Ends the burst open in another field than the one an arrow is pressed in, its step taken
 * (`stopArrowBurst`); the burst open in that field keeps on, its pause ended, the arrow held.
 *
 * @kind procedure
 * @param document - page document
 * @param field - the field the arrow is pressed in
 * @param step - asks the step of the history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function stopOtherBurst(document: Document, field: Element, step: () => void): void {
  const burst = ARROW_BURSTS.get(document);

  if (burst?.field === field) {
    clearTimeout(burst.pause);
  } else {
    stopArrowBurst(document, step);
  }
}

/**
 * Opens a burst at Up or Down pressed in a field whose arrows step a value (`readSteppedField`),
 * the arrow then held; a burst open in that field keeps on, its pause ended, one open in another
 * field ends first (`stopOtherBurst`).
 *
 * @kind procedure
 * @param document - page document
 * @param event - a key pressed in the panel
 * @param step - asks the step of the history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function startArrowBurst(document: Document, event: KeyboardEvent, step: () => void): void {
  const field = STEP_ARROWS.has(event.key) ? readSteppedField(event) : undefined;

  if (field === undefined) {
    return;
  }

  stopOtherBurst(document, field, step);
  ARROW_BURSTS.set(document, { field, pause: undefined });
}

/**
 * Starts the pause of the burst open at Up or Down released, wherever the focus is: the burst's
 * step taken at its end (`stopArrowBurst`), unless an arrow pressed meanwhile keeps it open.
 *
 * @kind procedure
 * @param document - page document
 * @param event - a key released
 * @param step - asks the step of the history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function startBurstPause(document: Document, event: KeyboardEvent, step: () => void): void {
  const burst = ARROW_BURSTS.get(document);

  if (burst === undefined || !STEP_ARROWS.has(event.key)) {
    return;
  }

  clearTimeout(burst.pause);

  const pause = setTimeout(() => {
    stopArrowBurst(document, step);
  }, BURST_PAUSE);

  ARROW_BURSTS.set(document, { field: burst.field, pause });
}

/**
 * Listens to the bursts of arrows in the panel's fields (US-081, SP-031's answer 5): Up or Down
 * pressed in a number field or a field of several shapes opens a burst (`startArrowBurst`), its
 * release starts a pause of 500 ms (`startBurstPause`), and the burst's step is taken at the
 * pause's end, unless an arrow pressed meanwhile joins the burst; the window losing the focus ends
 * it at once, a release in another window never heard (`stopArrowBurst`). A burst left open by the
 * page mounted before is forgotten, its step not taken.
 *
 * @kind procedure
 * @param document - page document
 * @param step - asks the step of the history (`schedulePanelStep`)
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToArrowBursts(document: Document, step: () => void): void {
  clearTimeout(ARROW_BURSTS.get(document)?.pause);
  ARROW_BURSTS.delete(document);

  selectElement(document, "panel").addEventListener("keydown", (event) => {
    startArrowBurst(document, event, step);
  });

  document.addEventListener(
    "keyup",
    (event) => {
      startBurstPause(document, event, step);
    },
    { signal: readListenerSignal(document) },
  );

  document.defaultView?.addEventListener(
    "blur",
    () => {
      stopArrowBurst(document, step);
    },
    { signal: readListenerSignal(document) },
  );
}
