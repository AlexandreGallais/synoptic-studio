// @vitest-environment happy-dom
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { basename, join } from "node:path";

import { describe, expect, it } from "vitest";

import {
  SVG_NAMESPACE,
  createEmbeddedFontsElements,
  createPathElement,
  createSvgElement,
  createTextElement,
  textDrawing,
} from "../src";

import { DECORATED_METRICS, HELLO_TEXT } from "./text-fixtures";

import type { EmbeddedFace, Text } from "../src";

/** Folders whose functions need a property test (ADR-0022). */
const PROPERTY_LAYERS = ["src/math", "src/geometry"];

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** A property in a test file: `test.prop(` or `it.prop(`. */
const PROPERTY = /\b(?:test|it)\.prop\(/u;

/** An exported function of the layers checked, with the test file beside it. */
type CheckedFunction = {
  readonly name: string;
  readonly test: string;
};

/**
 * Exported functions of a folder, each with the path of its own test file.
 *
 * @param folder - folder relative to the root
 * @returns one entry per `export function`, `index.ts` and tests left out
 */
function exportedFunctions(folder: string): readonly CheckedFunction[] {
  return readdirSync(join(ROOT, folder), { recursive: true })
    .map(String)
    .filter(
      (file) => file.endsWith(".ts") && basename(file) !== "index.ts" && !file.endsWith(".test.ts"),
    )
    .flatMap((file) =>
      readFileSync(join(ROOT, folder, file), "utf8")
        .matchAll(/^export function (?<name>\w+)/gmu)
        .map((match) => ({
          name: match.groups?.["name"] ?? "",
          test: join(ROOT, folder, file.replace(/\.ts$/u, ".test.ts")),
        }))
        .toArray(),
    );
}

/**
 * Whether a function's own test file holds a property (ADR-0022): a property written for another
 * function, even one calling it, does not count.
 *
 * @param checked - the function and its test file
 * @returns true when the test file exists and holds `test.prop` or `it.prop`
 */
function hasOwnProperty(checked: CheckedFunction): boolean {
  // A property commented out does not count.
  return (
    existsSync(checked.test) &&
    PROPERTY.test(readFileSync(checked.test, "utf8").replaceAll(/\/\/.*$/gmu, ""))
  );
}

/** Functions checked: every exported function of the layers. */
const CHECKED = PROPERTY_LAYERS.flatMap((layer) => exportedFunctions(layer));

/** Functions of the two layers at EN-020: the check may never read fewer (an empty list passes). */
const FUNCTIONS_AT_EN_020 = 41;

describe("fitness functions", () => {
  it("reads every function of the layers, not an empty list", () => {
    expect(CHECKED.length).toBeGreaterThanOrEqual(FUNCTIONS_AT_EN_020);

    expect(CHECKED.map((checked) => checked.name)).toEqual(
      expect.arrayContaining(["add", "mirrorAcrossAxis", "turningAngles"]),
    );
  });

  it("[F11.AC5] gives every math and geometry function a property test in its own test file", () => {
    expect(
      CHECKED.filter((checked) => !hasOwnProperty(checked)).map((checked) => checked.name),
    ).toEqual([]);
  });
});

/** Exported names of a module, read from its text. */
const EXPORTED = /^export (?:async )?(?:function|const) (?<name>\w+)/gmu;

/**
 * Names exported by the modules of a folder, read from their text.
 *
 * @param folder - path from the repository root
 * @returns each exported function or constant name
 */
function exportedNames(folder: string): readonly string[] {
  return readdirSync(join(ROOT, folder))
    .filter((name) => name.endsWith(".ts") && !name.endsWith(".test.ts"))
    .flatMap((name) =>
      readFileSync(join(ROOT, folder, name), "utf8")
        .matchAll(EXPORTED)
        .map((match) => match.groups?.["name"] ?? "")
        .toArray(),
    );
}

/** Names the shared test modules export: a test file never declares them again (F11). */
const SHARED_HELPERS = new Set(
  ["playground/user-actions", "tests"].flatMap((folder) => exportedNames(folder)),
);

/**
 * Text of every file of `src/` and `playground/` whose name ends as asked.
 *
 * @param ending - end of the file name, such as `.test.ts`
 * @returns path from the repository root, and text, of each file
 */
function sourcesEndingWith(
  ending: string,
): readonly { readonly path: string; readonly text: string }[] {
  return ["src", "playground"].flatMap((root) =>
    readdirSync(join(ROOT, root), { recursive: true })
      .map(String)
      .filter((name) => name.endsWith(ending))
      .map((name) => ({
        path: join(root, name),
        text: readFileSync(join(ROOT, root, name), "utf8"),
      })),
  );
}

/**
 * Number of times a text holds a piece.
 *
 * @param text - text to search
 * @param piece - exact piece of text
 * @returns how many times it appears
 */
function occurrences(text: string, piece: string): number {
  return text.split(piece).length - 1;
}

describe("a code base ready for curves (F11)", () => {
  it("[F11.AC1] declares each shared test helper once, never again in a test file", () => {
    expect(SHARED_HELPERS).toContain("typeValue");
    expect(SHARED_HELPERS).toContain("rectanglePoints");

    expect(
      sourcesEndingWith(".test.ts").flatMap(({ path, text }) =>
        text
          .matchAll(/^(?:function|const) (?<name>\w+)/gmu)
          .map((match) => match.groups?.["name"] ?? "")
          .filter((name) => SHARED_HELPERS.has(name))
          .map((name) => `${path}: ${name}`)
          .toArray(),
      ),
    ).toEqual([]);
  });

  it("[F11.AC2] exports one writer of path data, its separator and close command declared once", () => {
    const modules = sourcesEndingWith(".ts")
      .filter(({ path }) => path.startsWith("src") && !path.endsWith(".test.ts"))
      .map(({ text }) => text);
    const writers = modules.filter((text) => /^export function \w+ToPathData\(/mu.test(text));

    expect(writers).toHaveLength(1);
    expect(writers[0]).toContain("export function contourPiecesToPathData(");
    expect(modules.filter((text) => text.includes("const SEPARATOR ="))).toHaveLength(1);
    expect(modules.filter((text) => text.includes("const CLOSE_PATH ="))).toHaveLength(1);
  });

  it("[F11.AC3] computes each corner's triple and turning angle once for the fillets of a contour", () => {
    const text = readFileSync(join(ROOT, "src/geometry/corner-radius/fillet-arcs.ts"), "utf8");

    expect(occurrences(text, "cornerTriples(")).toBe(1);
    expect(occurrences(text, "turningAngles(")).toBe(1);
    expect(occurrences(text, "turningAngle(")).toBe(0);
  });
});

/** The SVG elements the library may write (ADR-0002, ADR-0052, ADR-0060, decision 2). */
const SVG_ELEMENTS = new Set(["defs", "g", "path", "style", "svg", "text", "tspan"]);

/** An element created in `src/` or `playground/`: `createElementNS(<namespace>, <name>)`. */
const CREATED = /createElementNS\(\s*(?<namespace>[\w.]+),\s*(?<name>[^)]+)\)/gu;

/** A comment: a block comment, or a line comment not part of a URL's `://`. */
const COMMENT = /\/\*[\s\S]*?\*\/|(?<!:)\/\/.*$/gmu;

/** A name written as a string literal, which alone can be checked: `"<name>"`. */
const LITERAL = /^"(?<name>[^"]+)"$/u;

/**
 * The name an element is created with, when written as a string literal.
 *
 * @param written - the argument as written
 * @param call - the whole call
 * @returns the literal's characters, or the whole call
 */
function literalName(written: string, call: string): string {
  return LITERAL.exec(written)?.groups?.["name"] ?? call;
}

/** An element created in the code: its name, its namespace's expression and its file. */
type Creation = { readonly name: string; readonly namespace: string; readonly path: string };

/**
 * An element created, read from its call: a name that is not a string literal stays the whole
 * call, which no list holds.
 *
 * @param match - the call matched by `CREATED`
 * @param path - its file
 * @returns its name, namespace and file
 */
function creation(match: RegExpMatchArray, path: string): Creation {
  const { name = "", namespace = "" } = match.groups ?? {};

  return { name: literalName(name, match[0]), namespace, path };
}

/**
 * Whether an element created with a namespace written as in the code is one ADR-0060 allows: an
 * SVG element of the list, and none of another namespace.
 *
 * @param namespace - the namespace's expression, `SVG_NAMESPACE` for an SVG element
 * @param name - the element's name
 * @returns true when it is allowed
 */
function isAllowedCreation(namespace: string, name: string): boolean {
  return namespace === "SVG_NAMESPACE" && SVG_ELEMENTS.has(name);
}

/**
 * Whether an element of a written SVG is one ADR-0060 allows: in the SVG namespace and the list.
 *
 * @param element - an element of the SVG written
 * @returns true when it is
 */
function isAllowedElement(element: Element): boolean {
  return element.namespaceURI === SVG_NAMESPACE && SVG_ELEMENTS.has(element.localName);
}

/**
 * Where an element of a written SVG breaks its place (ADR-0060, decision 2): an element outside
 * the SVG namespace or the list, or a `style` outside `defs`.
 *
 * @param element - an element of the SVG written
 * @returns its name and its parent's when misplaced, or nothing
 */
function misplacement(element: Element): string | undefined {
  const parent = String(element.parentElement?.localName);
  const isPlaced = element.localName !== "style" || parent === "defs";

  return isPlaced && isAllowedElement(element) ? undefined : `${element.localName} in ${parent}`;
}

/** A face embedded, for the document written. */
const EMBEDDED: EmbeddedFace = {
  family: "Arial",
  file: { bytes: Uint8Array.from([0, 1, 0, 0]), isEmbeddable: true, version: "" },
  isItalic: false,
  weight: 400,
};

/**
 * Texts of every kind the library writes: plain, turned, decorated, with a background, and one
 * with every option on: in a box, cut with its ellipsis, underlined, struck through, with a
 * background.
 */
const TEXTS: readonly Text[] = [
  HELLO_TEXT,
  { ...HELLO_TEXT, angle: 30, content: "Hello\nWorld", isStruckThrough: true, isUnderlined: true },
  { ...HELLO_TEXT, hasBackground: true, isInBox: true, boxHeight: 40, boxWidth: 50 },
  {
    ...HELLO_TEXT,
    boxHeight: 40,
    boxWidth: 30,
    content: "Hello World Hello World",
    hasBackground: true,
    isInBox: true,
    isStruckThrough: true,
    isUnderlined: true,
    maximumLines: 1,
  },
];

/**
 * A document as the library writes it: its canvas, its embedded fonts, a shape and its texts.
 *
 * @returns the `<svg>`
 */
function writtenDocument(): SVGSVGElement {
  const svg = createSvgElement(document, { height: 100, width: 100, x: 0, y: 0 });

  svg.append(
    ...createEmbeddedFontsElements(document, [EMBEDDED]),
    createPathElement(document, "M0 0 L10 0 L10 10 Z"),
    ...TEXTS.map((text) => createTextElement(document, text, textDrawing(DECORATED_METRICS, text))),
  );

  return svg;
}

describe("the SVG elements the library writes (ADR-0060)", () => {
  // The code without its comments, where a mention of the function is no call.
  const sources = sourcesEndingWith(".ts")
    .filter(({ path }) => !path.endsWith(".test.ts"))
    .map(({ path, text }) => ({ path, text: text.replaceAll(COMMENT, "") }));
  const created = sources.flatMap(({ path, text }) =>
    text
      .matchAll(CREATED)
      .map((match) => creation(match, path))
      .toArray(),
  );

  it("[F17.AC2] reads every call creating an element, whatever its spelling", () => {
    // `document["createElementNS"](…)` escapes the pattern: each mention must be a call it reads.
    expect(
      sources.reduce((count, { text }) => count + occurrences(text, "createElementNS"), 0),
    ).toBe(created.length);
  });

  it("reads every element created, not an empty list", () => {
    expect(created.map(({ name }) => name)).toEqual(
      expect.arrayContaining(["defs", "g", "path", "style", "svg", "text", "tspan"]),
    );
  });

  it("[F17.AC2] creates only the elements ADR-0060 allows, no switch nor foreignObject", () => {
    expect(
      created
        .filter(({ name, namespace }) => !isAllowedCreation(namespace, name))
        .map(({ name, path }) => `${path}: ${name}`),
    ).toEqual([]);
  });

  it("[F17.AC2] writes each element in its place: style in defs, no switch nor XHTML", () => {
    const svg = writtenDocument();
    const elements = [...svg.querySelectorAll("*")];

    expect(elements.map((element) => element.localName)).toEqual(
      expect.arrayContaining(["defs", "style", "path", "g", "text", "tspan"]),
    );

    expect(elements.map((element) => misplacement(element)).filter(Boolean)).toEqual([]);
    expect(svg.querySelectorAll("switch, foreignObject, div, span")).toHaveLength(0);
  });

  it("[F17.AC1] writes a text with every option on as one group: its text and its paths", () => {
    const text = TEXTS.at(-1) ?? HELLO_TEXT;
    const group = createTextElement(document, text, textDrawing(DECORATED_METRICS, text));

    expect(group.localName).toBe("g");

    // The background, then the text, then its underline and its strike-through.
    expect([...group.children].map((child) => child.localName)).toEqual([
      "path",
      "text",
      "path",
      "path",
    ]);

    // Cut to one line: "Hello" (36.46) is wider than 30, so "H" and three full stops, the face
    // having no "…".
    expect(group.querySelector("tspan")?.textContent).toBe("H...");
  });
});
