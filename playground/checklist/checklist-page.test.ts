// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { EPIC_15_CHECKLIST, EPIC_17_CHECKLIST } from "../checklists";
import { openServedPage, readDisplay, readText, selectPageElement } from "../user-actions";

/**
 * The check boxes of the checklist on the page, in order.
 *
 * @returns each line's box
 */
function readBoxes(): readonly HTMLInputElement[] {
  return [...selectPageElement("#checklist-lines").querySelectorAll("input")];
}

/**
 * Ticks a line's box as a click does: its state changed, then its change event sent.
 *
 * @param box - the line's box
 */
function tick(box: HTMLInputElement | undefined): void {
  expect(box).toBeDefined();
  box?.click();
}

beforeEach(() => {
  localStorage.clear();
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("[F17.AC8] the checklist page (US-051)", () => {
  it("lists every line of E15's checklist, one check box each, without Previous nor Next", () => {
    openServedPage("?checklist=E15");

    expect(readDisplay("checklist") === "none").toBe(false);
    expect(readText("#checklist-name")).toBe("E15");
    expect(readBoxes()).toHaveLength(EPIC_15_CHECKLIST.length);
    expect(readBoxes().every((box) => box.type === "checkbox" && !box.checked)).toBe(true);

    const first = selectPageElement("#checklist-lines li").textContent;

    expect(first).toContain(EPIC_15_CHECKLIST[0]?.todo);
    expect(first).toContain(EPIC_15_CHECKLIST[0]?.see);
    expect(first).toContain("F05");
    // The long guided test is not shown beside it: no Previous nor Next.
    expect(readDisplay("guide") === "none").toBe(true);
    expect(selectPageElement("#checklist").querySelector("button")).toBeNull();
  });

  it("is hidden without a checklist in the address, or with an unknown one", () => {
    openServedPage("");

    expect(readDisplay("checklist") === "none").toBe(true);

    openServedPage("?checklist=E99");

    expect(readDisplay("checklist") === "none").toBe(true);
    expect(readBoxes()).toHaveLength(0);
  });

  it("keeps the boxes ticked in that browser when the page is reloaded", () => {
    openServedPage("?checklist=E15");
    tick(readBoxes()[0]);
    tick(readBoxes()[3]);
    tick(readBoxes()[3]);
    tick(readBoxes()[5]);

    openServedPage("?checklist=E15");

    expect(readBoxes().map((box) => box.checked)).toEqual(
      Array.from(EPIC_15_CHECKLIST, (unused, index) => index === 0 || index === 5),
    );
  });

  it("works without the browser's storage, its ticks then lost at the reload", () => {
    // As a browser blocking the site's data: reading the storage itself throws.
    vi.spyOn(globalThis, "localStorage", "get").mockImplementation(() => {
      throw new Error("Storage blocked");
    });

    openServedPage("?checklist=E15");
    tick(readBoxes()[1]);

    expect(readBoxes()[1]?.checked).toBe(true);

    openServedPage("?checklist=E15");

    expect(readBoxes().some((box) => box.checked)).toBe(false);
  });

  it("goes on when the storage refuses to keep a tick, as a full one does", () => {
    // The page's storage, whose every write is refused.
    const full = new Proxy(localStorage, {
      get: (storage, property): unknown =>
        property === "setItem"
          ? (): never => {
              throw new Error("Quota exceeded");
            }
          : Reflect.get(storage, property),
    });

    vi.spyOn(globalThis, "localStorage", "get").mockReturnValue(full);

    openServedPage("?checklist=E15");
    tick(readBoxes()[2]);

    expect(readBoxes()[2]?.checked).toBe(true);
  });

  it("ignores a stored value that is not a list of lines", () => {
    localStorage.setItem("synoptic-studio.checklist.E15", "{not a list");
    openServedPage("?checklist=E15");

    expect(readBoxes().some((box) => box.checked)).toBe(false);

    localStorage.setItem("synoptic-studio.checklist.E15", '{"todo":1}');
    openServedPage("?checklist=E15");

    expect(readBoxes().some((box) => box.checked)).toBe(false);
  });
});

describe("[F17.AC8] E15's checklist (US-051)", () => {
  const text = EPIC_15_CHECKLIST.map((line) => `${line.todo} ${line.see}`).join("\n");

  it("has about 50 lines, each with its own thing to try, from every feature of E15", () => {
    expect(EPIC_15_CHECKLIST.length).toBeGreaterThanOrEqual(36);
    expect(EPIC_15_CHECKLIST.length).toBeLessThanOrEqual(56);

    const todos = new Set(EPIC_15_CHECKLIST.map((line) => line.todo));

    expect(todos.size).toBe(EPIC_15_CHECKLIST.length);

    expect(new Set(EPIC_15_CHECKLIST.map((line) => line.feature))).toEqual(
      new Set(["F05", "F14", "F15", "F16", "F17"]),
    );
  });

  it.each([
    "Font list",
    "Import a font",
    "Installed fonts",
    "Weight",
    "Bold",
    "Italic",
    "Size",
    "Color",
    "picker",
    "Letter spacing",
    "word spacing",
    "Underline",
    "Strike-through",
    "Case",
    "Right to left",
    "Alignment",
    "Fit Box",
    "Fit content",
    "Max lines",
    "Box Reset",
    "Ellipsis",
    "Vertical alignment",
    "Line height",
    "Trim",
    "Background",
    "Angle",
    "Embed fonts",
    "Export SVG",
    "HSL",
    "hold the Up arrow",
    "turned frame",
  ])("names the option kept: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each([
    "Reading to confirm (US-046)",
    "Reading to confirm (US-047)",
    "Reading to confirm (US-049)",
    "Question (AUD-019): Installed fonts, Helvetica Neue",
    "Question (AUD-019): in Safari",
    "Reading to confirm (US-052)",
  ])("names VAL-018's reading, question or warning: %s", (line) => {
    expect(text).toContain(line);
  });

  it.each(["balanced", "Wrap", "HTML layer", "at a character", "Break"])(
    "names no option removed by F17: %s",
    (option) => {
      expect(text).not.toContain(option);
    },
  );
});

describe("[F18.AC12] [F19.AC8] E17's checklist on the page (VAL-019, VAL-020)", () => {
  it("lists F18's 12 lines then F19's 12, one check box each, each with its own thing to try", () => {
    openServedPage("?checklist=E17");

    expect(readText("#checklist-name")).toBe("E17");
    expect(readBoxes()).toHaveLength(24);

    const todos = new Set(EPIC_17_CHECKLIST.map((line) => line.todo));

    expect(todos.size).toBe(24);

    expect(EPIC_17_CHECKLIST.map((line) => line.feature)).toEqual([
      ...Array.from({ length: 12 }, () => "F18"),
      ...Array.from({ length: 12 }, () => "F19"),
    ]);
  });

  it.each(["What to see", "to write", "Helvetica.ttc", "Apple Color Emoji", "/System/Library"])(
    "holds no draft line nor macOS font file: %s",
    (word) => {
      expect(EPIC_17_CHECKLIST.map((line) => `${line.todo} ${line.see}`).join("\n")).not.toContain(
        word,
      );
    },
  );
});

describe("[F18.AC12] E17's checklist, F18's lines (VAL-019)", () => {
  const text = EPIC_17_CHECKLIST.filter((line) => line.feature === "F18")
    .map((line) => `${line.todo} ${line.see}`)
    .join("\n");

  it.each([
    "npm run test in two terminals",
    "heavy lock held by process",
    "four subagents",
    "npm run worktree:new",
    "npm run backlog:index",
    "merge: union",
    "npm run metrics:session",
    "status line",
    "npm run epic:check",
    "npm run mutate",
    "Installed fonts",
    "Embed fonts",
  ])("names the tool or option to try: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each([
    "Decision to confirm (ADR-0068)",
    "Decision to confirm (US-061)",
    "Decision to confirm (EN-067)",
    "Decision to confirm (ADR-0067)",
  ])("names the decision to confirm: %s", (line) => {
    expect(text).toContain(line);
  });
});

describe("[F19.AC8] E17's checklist, F19's lines (VAL-020)", () => {
  const text = EPIC_17_CHECKLIST.filter((line) => line.feature === "F19")
    .map((line) => `${line.todo} ${line.see}`)
    .join("\n");

  it.each([
    "npm run lint",
    "TS2322",
    "Unused oxlint-disable directive",
    "markdown/table-column-count",
    "npm run format:check",
    "npm run format",
    "Version 7.0.2",
    "npm run build",
    "npm run docs:api",
    "skipped: nothing changed in their inputs",
    "npm run check:rust",
    "check for Windows",
    "eslint-rules-accounted",
    "cargo test --release -q -p lint",
  ])("names the command or the output to see: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each(["eslint .", "prettier", "tsc6"])("names no tool F19 removed: %s", (tool) => {
    expect(text.toLowerCase()).not.toContain(tool);
  });
});
