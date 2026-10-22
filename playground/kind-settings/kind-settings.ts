/**
 * The settings a new shape takes from the last shape of its kind (SP-026), by kind: the inputs
 * holding them, as a CSS selector, in the order the page holds them. A rectangle keeps its corner
 * radius, a polygon its radius, corners, stretched option and anchor; a round shape has none but
 * its box. A text's options are fields no other kind writes, so the panel keeps them as the last
 * text left them.
 */
export const KIND_SETTINGS = new Map([
  ["polygon", '#corners, #stretched, input[name="anchor"], #radius'],
  ["rectangle", "#radius"],
]);
