// @vitest-environment happy-dom
import { NgZone, ɵNoopNgZone as NoopNgZone } from "@angular/core";
import { TestBed } from "@angular/core/testing";
import { describe, expect, it } from "vitest";

import { loadPage } from "../../testing";

/** Every id of the page before the Angular app, `playground/index.html`'s, sorted. */
const OLD_IDS = [
  "add-midpoint",
  "anchor-field",
  "anchor-grid",
  "angle",
  "canvas",
  "color-hue",
  "color-hue-thumb",
  "color-hue-value",
  "color-level-name",
  "color-level-value",
  "color-model",
  "color-opacity",
  "color-opacity-thumb",
  "color-picker",
  "color-plane",
  "color-plane-mark",
  "color-saturation-value",
  "color-values",
  "contour",
  "convert",
  "convert-hint",
  "corners",
  "corners-field",
  "corners-heading",
  "corners-section",
  "delete-message",
  "delete-points",
  "document-options",
  "effective",
  "embed-fonts",
  "export-svg",
  "font-file",
  "font-file-button",
  "font-first",
  "font-left-out",
  "font-message",
  "font-sources",
  "handle-after-x",
  "handle-after-y",
  "handle-before-x",
  "handle-before-y",
  "handle-fields",
  "handle-method",
  "handle-mode",
  "handles-switch",
  "height",
  "history-list",
  "history-memory",
  "history-moves",
  "history-redo",
  "history-size",
  "history-steps",
  "history-steps-box",
  "history-undo",
  "installed-fonts",
  "installed-fonts-progress",
  "mode-points",
  "mode-shape",
  "model",
  "output",
  "overlay",
  "panel",
  "path-data",
  "pieces",
  "placement",
  "point-fields",
  "point-radius",
  "point-radius-field",
  "point-x",
  "point-y",
  "points",
  "polygon-heading",
  "polygon-options",
  "radius",
  "radius-field",
  "several-fields",
  "several-height",
  "several-width",
  "several-x",
  "several-y",
  "shape",
  "shape-panel",
  "shape-title",
  "shape-x",
  "shape-y",
  "size-heading",
  "size-section",
  "stage",
  "status",
  "straight-box",
  "stretched",
  "stretched-field",
  "text-alignment",
  "text-background",
  "text-background-color",
  "text-background-color-fields",
  "text-background-color-message",
  "text-background-color-swatch",
  "text-background-opacity",
  "text-bold",
  "text-box-fields",
  "text-box-height",
  "text-box-reset",
  "text-box-width",
  "text-case",
  "text-color",
  "text-color-fields",
  "text-color-message",
  "text-color-swatch",
  "text-content",
  "text-direction",
  "text-ellipsis",
  "text-fit",
  "text-font",
  "text-heading",
  "text-italic",
  "text-letter-spacing",
  "text-line-height",
  "text-maximum-lines",
  "text-opacity",
  "text-options",
  "text-size",
  "text-strike",
  "text-strike-color",
  "text-strike-color-fields",
  "text-strike-color-message",
  "text-strike-color-swatch",
  "text-strike-follows",
  "text-strike-opacity",
  "text-style",
  "text-trim",
  "text-underline",
  "text-underline-color",
  "text-underline-color-fields",
  "text-underline-color-message",
  "text-underline-color-swatch",
  "text-underline-follows",
  "text-underline-opacity",
  "text-vertical-alignment",
  "text-weight",
  "text-word-spacing",
  "tool-output",
  "tool-polygon",
  "tool-rectangle",
  "tool-reset",
  "tool-round",
  "tool-selector",
  "tool-text",
  "toolbar",
  "tools",
  "turn",
  "turn-left",
  "turn-right",
  "width",
  "zoom-actual",
  "zoom-box",
  "zoom-field",
  "zoom-fit-drawing",
  "zoom-fit-selection",
  "zoom-in",
  "zoom-menu",
  "zoom-out",
  "zoom-percentage",
];

/**
 * Every role of the page before the Angular app, sorted: its toolbar's and panel's groups, its
 * colour picker's sliders, its status lines.
 */
const OLD_ROLES = [
  ...Array.from({ length: 6 }, () => "group"),
  ...Array.from({ length: 3 }, () => "slider"),
  ...Array.from({ length: 7 }, () => "status"),
];

/**
 * The values of an attribute on every element of the document holding it, sorted.
 *
 * @param name - the attribute read, such as `id`
 * @returns each value, as many times as elements hold it
 */
function attributeValues(name: string): readonly string[] {
  return Array.from(
    document.querySelectorAll(`[${CSS.escape(name)}]`),
    (element) => element.getAttribute(name) ?? "",
  ).toSorted((first, second) => first.localeCompare(second));
}

describe("AppComponent", () => {
  it("[F50.AC4] holds every id and role of the page before it, zoneless", () => {
    loadPage("");

    // The 163 ids and 16 roles of the old page's markup, its script tag apart, none added.
    expect(attributeValues("id")).toEqual(OLD_IDS);
    expect(attributeValues("role")).toEqual(OLD_ROLES);
    // No zone.js: Angular runs zoneless, its zone a no-op.
    expect(Reflect.has(globalThis, "Zone")).toBe(false);
    expect(TestBed.inject(NgZone)).toBeInstanceOf(NoopNgZone);
  });
});
