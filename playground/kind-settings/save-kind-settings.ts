import { selectElement } from "../page";

import { KIND_SETTINGS } from "./kind-settings";

import type { KindMemory } from "./kind-memory";

/**
 * Keeps the settings of the shape the panel leaves, as the last shape of its kind: each
 * input of `KIND_SETTINGS` for that kind, its value and whether it is ticked, and the radius of
 * each vertex when they differ, kept as JSON on the toolbar under the key `<kind>Settings`. A kind
 * without settings, or the free shape, keeps nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param kind - the kind of the shape the panel leaves
 */
export function saveKindSettings(document: Document, kind: string): void {
  const selector = KIND_SETTINGS.get(kind);

  if (selector === undefined) {
    return;
  }

  const inputs = [...document.querySelectorAll<HTMLInputElement>(selector)];
  const memory: KindMemory = {
    radii: selectElement(document, "radius").dataset["radii"] ?? "",
    settings: inputs.map(({ checked, value }) => ({ checked, value })),
  };

  selectElement(document, "toolbar").dataset[`${kind}Settings`] = JSON.stringify(memory);
}
