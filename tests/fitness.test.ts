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

import { CHECK_ALL_LANES } from "./check-all-lanes";
import { isPlaygroundProject } from "./playground-project";
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
      .filter((name) => name.endsWith(ending) && !isPlaygroundProject(join(ROOT, root, name)))
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

/**
 * A per-test limit, `{ timeout: 1000 }` or with a constant of the same file, outside a comment,
 * its tags before it when it has some (`{ tags: ["timing"], timeout: 960 }`).
 */
const TIMEOUT = /^(?!\s*(?:\/\/|\/\*|\*)).*\{ (?:tags: \[[^\]]*\], )?timeout: (?<limit>\w+) \}/u;

/** The tag of a timing test, run alone after the other checks (`vite.config.ts`, REV-008). */
const TIMING_TAG = 'tags: ["timing"]';

/** Vitest's own limit per test, in ms: a test given a shorter one measures its speed. */
const VITEST_TIMEOUT = 5000;

/** A test's duration alone, written beside its limit: `alone: 210 ms` (EN-129). */
const ALONE = /\balone: (?<measure>\d[\d ]*) ms\b/u;

/** The line opening a test or a suite: `it(`, `it.each(…)(`, `test(`, `describe(`. */
const TEST_OPENING = /^\s*(?:it|test|describe)\b[\w.]*\(/u;

/** How many times its duration alone a limit is at least (`.claude/rules/tooling.md`). */
const LIMIT_FACTOR = 4;

/**
 * Limits of `src/`, `tests/` and `playground/` at AUD-031, 17 of EN-129 and 4 of the playground:
 * the check may never read fewer (an empty list passes).
 */
const LIMITS_AT_AUD_031 = 21;

/** A per-test limit of a test file, with what its comment says. */
type TimingLimit = {
  readonly at: string;
  readonly limit: number | undefined;
  readonly measure: number | undefined;
};

/**
 * Text of every test file of `src/`, `tests/` and `playground/`, the three folders holding a
 * per-test limit (AUD-031).
 *
 * @returns path from the repository root, and text, of each file
 */
function timedTestFiles(): readonly { readonly path: string; readonly text: string }[] {
  return ["src", "tests", "playground"].flatMap((root) =>
    readdirSync(join(ROOT, root), { recursive: true })
      .map(String)
      .filter((name) => name.endsWith(".test.ts") && !isPlaygroundProject(join(ROOT, root, name)))
      .map((name) => ({
        path: join(root, name),
        text: readFileSync(join(ROOT, root, name), "utf8"),
      })),
  );
}

/**
 * Value of a limit: a number, written with `_` or not, or a constant the file declares.
 *
 * @param written - what follows `timeout:`
 * @param text - text of the test file
 * @returns the limit in milliseconds, undefined when it cannot be read
 */
function limitValue(written: string, text: string): number | undefined {
  const declaration = new RegExp(`^const ${written} = (?<value>\\d[\\d_]*);`, "mu");
  const declared = declaration.exec(text);
  const literal = /^\d[\d_]*$/u.test(written) ? written : declared?.groups?.["value"];

  return literal === undefined ? undefined : Number(literal.replaceAll("_", ""));
}

/**
 * Lines beside a limit: from the comment above the test opening it down to the limit's line.
 *
 * @param lines - lines of the test file
 * @param index - index of the limit's line
 * @returns the lines its measure may be written on
 */
function besideLimit(lines: readonly string[], index: number): readonly string[] {
  const opening = lines.findLastIndex((line, at) => at <= index && TEST_OPENING.test(line));
  const start = opening === -1 ? index : opening;
  const above = lines.slice(0, start).findLastIndex((line) => !/^\s*\/\//u.test(line));

  return lines.slice(above + 1, index + 1);
}

/**
 * Every per-test limit of a test file, with its value and the measure written beside it.
 *
 * @param path - path of the file from the repository root
 * @param text - text of the file
 * @returns one entry per `timeout:`, located by file and line
 */
function timingLimits(path: string, text: string): readonly TimingLimit[] {
  const lines = text.split("\n");

  return lines
    .map((line, index) => ({ index, written: TIMEOUT.exec(line)?.groups?.["limit"] }))
    .filter(({ written }) => written !== undefined)
    .map(({ index, written }) => {
      const measure = besideLimit(lines, index)
        .map((beside) => ALONE.exec(beside)?.groups?.["measure"])
        .find((found) => found !== undefined);

      return {
        at: `${path}:${String(index + 1)}`,
        limit: limitValue(written ?? "", text),
        measure: measure === undefined ? undefined : Number(measure.replaceAll(" ", "")),
      };
    });
}

/**
 * What is wrong with a limit, if anything.
 *
 * @param timing - the limit and its measure
 * @returns the fault, naming the file and the line, or undefined when the limit holds
 */
function limitFault(timing: TimingLimit): string | undefined {
  if (timing.limit === undefined) {
    return `${timing.at}: a limit that is no number nor constant of the file`;
  }

  if (timing.measure === undefined) {
    return `${timing.at}: a limit of ${String(timing.limit)} ms without its "alone: … ms"`;
  }

  return timing.limit < LIMIT_FACTOR * timing.measure
    ? `${timing.at}: ${String(timing.limit)} ms, under four times ${String(timing.measure)} ms alone`
    : undefined;
}

/**
 * The timing tests of a file without the `timing` tag: a limit under Vitest's 5 s measures a
 * speed, which the other checks' load would slow, so the test runs alone (REV-008).
 *
 * @param path - the file's path, to name it
 * @param text - the file's text
 * @returns each such limit, by its file and its line
 */
function untaggedTimings(path: string, text: string): readonly string[] {
  return text
    .split("\n")
    .map((line, index) => ({
      at: `${path}:${String(index + 1)}`,
      isTagged: line.includes(TIMING_TAG),
      value: limitValue(TIMEOUT.exec(line)?.groups?.["limit"] ?? "", text),
    }))
    .filter(({ isTagged, value }) => !isTagged && (value ?? VITEST_TIMEOUT) < VITEST_TIMEOUT)
    .map(({ at, value }) => `${at}: a limit of ${String(value)} ms without the timing tag`);
}

/**
 * Compares two paths by their names.
 *
 * @param left - one path
 * @param right - another
 * @returns their order
 */
function byName(left: string, right: string): number {
  return left.localeCompare(right);
}

/**
 * A per-test limit of a made-up test file, written so that this file's own scan reads none.
 *
 * @param written - what follows `timeout:`
 * @returns `{ timeout: <written> }`
 */
function limit(written: string): string {
  return `{ timeout: ${written} }`;
}

describe("the timing tests (EN-129)", () => {
  const limits = timedTestFiles().flatMap(({ path, text }) => timingLimits(path, text));

  it("reads every limit of src, tests and playground, not an empty list", () => {
    expect(limits.length).toBeGreaterThanOrEqual(LIMITS_AT_AUD_031);
  });

  it("[F29.AC1] gives every limit its measure alone, the limit at least four times it", () => {
    expect(limits.map((timing) => limitFault(timing)).filter(Boolean)).toEqual([]);
  });

  it("tags every limit under Vitest's 5 s as a timing test, run alone", () => {
    expect(timedTestFiles().flatMap(({ path, text }) => untaggedTimings(path, text))).toEqual([]);
  });

  it("runs alone every file holding a timing test, and no other", () => {
    const script = CHECK_ALL_LANES.find(({ name }) => name === "test:timings")?.command ?? "";
    const named = script.split(" ").filter((word) => word.endsWith(".test.ts"));

    const tagged = timedTestFiles()
      .filter(({ text }) =>
        text.split("\n").some((line) => TIMEOUT.test(line) && line.includes(TIMING_TAG)),
      )
      .map(({ path }) => path);

    expect(named.toSorted(byName)).toEqual(tagged.toSorted(byName));
  });

  it("names each timing test of a made-up file without its tag", () => {
    // 4 999 ms untagged; 4 999 tagged; 5 000, Vitest's own, needs none; `LONG` is 1 200 ms.
    const tagged = `{ ${TIMING_TAG}, timeout: 4999 }`;
    const text = [
      "const LONG = 1_200;",
      `it("a", () => {}, ${limit("4999")});`,
      `it("b", () => {}, ${tagged});`,
      `it("c", () => {}, ${limit("5000")});`,
      `it("d", () => {}, ${limit("LONG")});`,
    ].join("\n");

    expect(untaggedTimings("t.test.ts", text)).toEqual([
      "t.test.ts:2: a limit of 4999 ms without the timing tag",
      "t.test.ts:5: a limit of 1200 ms without the timing tag",
    ]);
  });

  // A made-up test file (AUD-031, F3): the limits of the repository all hold, so only this one
  // shows each fault found, and a factor other than four failing.
  const madeUp = [
    "const LONG = 1_200;",
    "",
    'describe("a suite", () => {',
    "  // alone: 100 ms, three times it.",
    `  it("is under four times", () => {}, ${limit("300")});`,
    "",
    `  it("has no measure", () => {}, ${limit("500")});`,
    "",
    "  // alone: 300 ms, four times it.",
    `  it("holds at four times exactly", () => {}, ${limit("LONG")});`,
    "",
    "  // alone: 300 ms.",
    `  it("reads a constant the file lacks", () => {}, ${limit("MISSING")});`,
    "",
    "  // alone: 1 000 ms, a constant at 1.2 times it.",
    `  it("reads a constant under four times", () => {}, ${limit("LONG")});`,
    "});",
  ].join("\n");

  it("[F29.AC1] reads each limit of a made-up file with its value and its measure", () => {
    // Lines 5, 7, 10, 13 and 16; `LONG` is 1 200 ms, `MISSING` declared nowhere.
    expect(timingLimits("tests/made-up.test.ts", madeUp)).toEqual([
      { at: "tests/made-up.test.ts:5", limit: 300, measure: 100 },
      { at: "tests/made-up.test.ts:7", limit: 500, measure: undefined },
      { at: "tests/made-up.test.ts:10", limit: 1200, measure: 300 },
      { at: "tests/made-up.test.ts:13", limit: undefined, measure: 300 },
      { at: "tests/made-up.test.ts:16", limit: 1200, measure: 1000 },
    ]);
  });

  it("[F29.AC1] names the file and the line of each fault of a made-up file", () => {
    // 300 < 4 × 100 = 400; 500 without a measure; 1 200 = 4 × 300 holds; MISSING unread;
    // 1 200 < 4 × 1 000 = 4 000.
    expect(
      timingLimits("tests/made-up.test.ts", madeUp).map((timing) => limitFault(timing)),
    ).toEqual([
      "tests/made-up.test.ts:5: 300 ms, under four times 100 ms alone",
      'tests/made-up.test.ts:7: a limit of 500 ms without its "alone: … ms"',
      undefined,
      "tests/made-up.test.ts:13: a limit that is no number nor constant of the file",
      "tests/made-up.test.ts:16: 1200 ms, under four times 1000 ms alone",
    ]);
  });
});
