import { selectElement } from "../page";

import { KIND_SETTINGS } from "./kind-settings";

import type { KindMemory } from "./kind-memory";

/**
 * Reads the settings a new shape of a kind starts with: those `saveKindSettings` kept for the
 * last shape of that kind, otherwise the values the page opens with, as `index.html` writes them,
 * one radius for every vertex.
 *
 * @kind procedure
 * @param document - page document
 * @param inputs - the inputs of the kind's settings, in the order the page holds them
 * @param kind - the kind of the new shape
 * @returns the radii and one setting per input, in the same order
 * @see REF-INKSCAPE-TUTORIAL-SHAPES
 */
function readKindMemory(
  document: Document,
  inputs: readonly HTMLInputElement[],
  kind: string,
): KindMemory {
  const kept = selectElement(document, "toolbar").dataset[`${kind}Settings`];

  if (kept === undefined) {
    return {
      radii: "",
      settings: inputs.map((input) => ({
        checked: input.defaultChecked,
        value: input.defaultValue,
      })),
    };
  }

  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by saveKindSettings only, from the same inputs
  return JSON.parse(kept) as KindMemory;
}

/**
 * Writes the radius of each vertex kept on the radius field, or none, so that the radius typed
 * goes to every vertex.
 *
 * @kind procedure
 * @param document - page document
 * @param radii - the radii, a space between two; empty for none
 * @see REF-INKSCAPE-TUTORIAL-SHAPES
 */
function writeKeptRadii(document: Document, radii: string): void {
  const { dataset } = selectElement(document, "radius");

  if (radii === "") {
    delete dataset["radii"];
  } else {
    dataset["radii"] = radii;
  }
}

/**
 * Gives a new shape the settings of the last shape of its kind, or, before any, the values the
 * page opens with (SP-026), without firing any event: one memory per kind, so that a rectangle's
 * radius never reaches a polygon. A kind without settings changes nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param kind - the kind of the new shape
 * @see REF-INKSCAPE-TUTORIAL-SHAPES
 */
export function writeKindSettings(document: Document, kind: string): void {
  const selector = KIND_SETTINGS.get(kind);

  if (selector === undefined) {
    return;
  }

  const inputs = [...document.querySelectorAll<HTMLInputElement>(selector)];
  const memory = readKindMemory(document, inputs, kind);

  for (const [index, input] of inputs.entries()) {
    Object.assign(input, memory.settings[index]);
  }

  writeKeptRadii(document, memory.radii);
}
