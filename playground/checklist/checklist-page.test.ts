// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { EPIC_15_CHECKLIST, EPIC_17_CHECKLIST, EPIC_2_CHECKLIST } from "../checklists";
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

describe("[F18.AC12] [F19.AC8] E17's checklist on the page (VAL-019 to VAL-023)", () => {
  it("lists F18's 12 lines, F19's 12, F20's 12, F21's 12, then F22's 12, one check box each, each with its own thing to try", () => {
    openServedPage("?checklist=E17");

    expect(readText("#checklist-name")).toBe("E17");
    expect(readBoxes()).toHaveLength(60);

    const todos = new Set(EPIC_17_CHECKLIST.map((line) => line.todo));

    expect(todos.size).toBe(60);

    expect(EPIC_17_CHECKLIST.map((line) => line.feature)).toEqual([
      ...Array.from({ length: 12 }, () => "F18"),
      ...Array.from({ length: 12 }, () => "F19"),
      ...Array.from({ length: 12 }, () => "F20"),
      ...Array.from({ length: 12 }, () => "F21"),
      ...Array.from({ length: 12 }, () => "F22"),
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

describe("E17's checklist, F20's lines (VAL-021)", () => {
  const text = EPIC_17_CHECKLIST.filter((line) => line.feature === "F20")
    .map((line) => `${line.todo} ${line.see}`)
    .join("\n");

  it.each([
    "npm run context:size",
    "grep '^model:' .claude/agents/*.md",
    "Decision to confirm (EN-066)",
    "check-handback:",
    "status line",
    "npm run metrics:session -- --run",
    "npm run run:should-stop",
    "the limit is 90 %",
    "A question parks its story",
    "npm run git:sync",
    "npm run git:commit",
    "The library holds this page",
    "npm run library -- stats",
    "F20.AC8]",
  ])("names the command or the output to see: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each(["progress page", "progress:page", "claude.ai"])(
    "names nothing of the progress page F20 dropped: %s",
    (word) => {
      expect(text).not.toContain(word);
    },
  );
});

describe("E17's checklist, F21's lines (VAL-022)", () => {
  const text = EPIC_17_CHECKLIST.filter((line) => line.feature === "F21")
    .map((line) => `${line.todo} ${line.see}`)
    .join("\n");

  it.each([
    "ls .claude/hooks",
    "target/release/<binary>",
    "A worktree refuses a chained Git command",
    "npm run metrics",
    "no row for run-totals status",
    "ls scripts",
    "npm run -s library -- search fts5",
    "devDependencies",
    "npm run deps:unused",
    "target/release/git-hooks commit-msg",
    "subject may not be empty [subject-empty]",
    "npm run -s lint:secrets",
    "release-please",
    "npm run release:prepare",
    "57 line(s) drafted",
    "npm run -s page -- REF-CC-HOOKS",
    "npm run check:all",
    "f21_ac9_",
  ])("names the command or the output to see: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each([
    "Decision to confirm (ADR-0078)",
    "Decision to confirm (EN-104)",
    "Decision to confirm (EN-102)",
    "Decision to confirm (ADR-0077)",
    "Decision to confirm (ADR-0079)",
  ])("names the decision to confirm: %s", (line) => {
    expect(text).toContain(line);
  });

  it.each(["husky install", "npx secretlint", "npx knip", "npx ncu", "node scripts/library"])(
    "names no tool F21 removed: %s",
    (tool) => {
      expect(text).not.toContain(tool);
    },
  );
});

describe("E17's checklist, F22's lines (VAL-023)", () => {
  const text = EPIC_17_CHECKLIST.filter((line) => line.feature === "F22")
    .map((line) => `${line.todo} ${line.see}`)
    .join("\n");

  it.each([
    "GITHUB_ACTIONS=true",
    "TS2322",
    "--format agent",
    "cargo metadata failed (exit 101)",
    "pull request develop → main",
    "0.0.0-development",
    "ls CHANGELOG.md",
    "workflow_dispatch",
    "target/release/release notes",
    "57 lines",
    "target/release/release check",
    "no heading but its groups'",
    "f22_ac3_",
    "dependabot",
    "Run workflow on main",
    "v0.3.0 on main's head",
    "--merge",
    "One agent at a time",
    "f22_ac4_",
    "_review.md",
    "F22.AC5",
  ])("names the command or the output to see: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each([
    "Decision to confirm (EN-116): the format named",
    "Decision to confirm (EN-116): the CI fetches",
    "Decision to confirm (EN-117): release prepare is gone",
    "Decision to confirm (EN-117): a body's release text ends",
    "Decision to confirm (EN-117): only the library's",
    "Decision to confirm (AUD-024)",
  ])("names the decision to confirm: %s", (line) => {
    expect(text).toContain(line);
  });

  it.each(["pending until that review"])(
    "leaves the runs only GitHub shows for the next review: %s",
    (word) => {
      expect(text.split(word)).toHaveLength(3);
    },
  );

  it.each(["release:prepare", "release/0.3.0", "CHANGELOG.md:", "chore: release", "epic's end"])(
    "names nothing of the flow F22 replaced: %s",
    (word) => {
      expect(text).not.toContain(word);
    },
  );
});

describe("[F35.AC1] E02's checklist, F35's lines (VAL-036)", () => {
  const text = EPIC_2_CHECKLIST.filter((line) => line.feature === "F35")
    .map((line) => `${line.todo} ${line.see}`)
    .join("\n");

  it("lists F35's 11 lines, then F36's 12, one check box each, each with its own thing to try", () => {
    openServedPage("?checklist=E02");

    expect(readText("#checklist-name")).toBe("E02");
    expect(readBoxes()).toHaveLength(23);

    const todos = new Set(EPIC_2_CHECKLIST.map((line) => line.todo));

    expect(todos.size).toBe(23);

    expect(EPIC_2_CHECKLIST.map((line) => line.feature)).toEqual([
      ...Array.from({ length: 11 }, () => "F35"),
      ...Array.from({ length: 12 }, () => "F36"),
    ]);
  });

  it.each([
    "Reset",
    "Rectangle",
    "Polygon",
    "Round",
    "Text",
    "Selector pressed again",
    "click the new rectangle again",
    "the new rectangle still edited",
    "100 × 100 centred on the click",
    "above the first, which does not change",
    "Radius 10",
    "6 corners",
    "Corners 3",
    "Anchor Bottom center",
    "Text selected in Content",
    "Zoom the browser to 110 %",
    "whole numbers",
    "Empty Width",
    "Convert to free shape",
    "Inter 32 Regular",
    "Export SVG",
    "in the order drawn",
    "place-shape.test.ts",
  ])("names the tool, the option or what to see: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each([
    "Decision to confirm (SP-026): a new shape takes the settings of the last of its kind",
    "Decision to confirm (SP-026): a new text's content Text, selected",
    "Decision to confirm (US-063): a tool's button only arms the tool",
    "Decision to confirm (US-063): the Selector places nothing, a click with it picking a shape since F36",
    "Decision to confirm (US-063): the corner rounded to the nearest integer, a half up",
    "Decision to confirm (US-063): a shape refused is not kept",
    "Decision to confirm (US-063): a rectangle or a polygon made a free shape keeps its settings",
    "Decision to confirm (US-063, AUD-037): Reset and each guided step forget",
    "Decision to confirm (US-063): a text placed while no font measures it",
    "Decision to confirm (AUD-037): a kept text whose font is no longer read",
  ])("names the decision to confirm: %s", (line) => {
    expect(text).toContain(line);
  });

  it.each(["What to see", "to write", "turns the shape into", "drag", "Delete", "until F36"])(
    "holds no draft line nor what F35 removed or left to F36: %s",
    (word) => {
      expect(text).not.toContain(word);
    },
  );
});

describe("[F36.AC1] [F36.AC2] [F36.AC3] E02's checklist, F36's lines (VAL-037)", () => {
  const text = EPIC_2_CHECKLIST.filter((line) => line.feature === "F36")
    .map((line) => `${line.todo} ${line.see}`)
    .join("\n");

  it.each([
    "drag down and to the right",
    "a drag up and to the left",
    "whole numbers",
    "Selector pressed again",
    "Shift held",
    "Shift pressed then released",
    "a circle",
    "Fit Box",
    "press and release on one point",
    "100 × 100 centred on the point pressed",
    "a drag straight down",
    "Width 0",
    "a band 2 pixels wide along the edge",
    "the circle where both overlap",
    "nothing in the rounded corner nor outside the circle",
    "nothing with Rectangle pressed",
    "between the two words",
    "outlined as selected",
    "Radius 30",
    "stays on top",
    "the new polygon above all",
    "Weight 700 Bold, Size 40",
    "its font chosen in the list",
    "not the rectangle under the pointer",
    "No shape selected",
    "its fields hidden",
    "Export SVG not shown",
  ])("names the tool, the option or what to see: %s", (option) => {
    expect(text).toContain(option);
  });

  it.each([
    "Decision to confirm (US-064): the press and the pointer each rounded to the nearest integer",
    "the box from the smaller to the larger of each coordinate",
    "Decision to confirm (US-064): with Shift the side is the larger distance",
    "Decision to confirm (US-064): a text is never squared",
    "Decision to confirm (US-064): a release on the press's point is a click",
    "Decision to confirm (US-064): a side of 0 kept as drawn",
    "Decision to confirm (EN-143): a shape of width 0 under the pointer within 4 pixels",
    "Decision to confirm (EN-143): a text is under the pointer anywhere in its block",
    "Decision to confirm (US-065): the band along the contour as drawn, only with the Selector",
    "Decision to confirm (US-067): the selection shown whatever tool is armed",
    "a click picks only when pressed with the Selector, again on the shape selected",
    "Decision to confirm (US-067): none selected hides the fields",
    "To settle (US-067): Export SVG cannot be reached with none selected",
    "Question (US-067)",
    "(1) load it anyway",
    "(2) select it without loading its family",
    "(3) show it selected with a message",
  ])("names the decision to confirm or the question: %s", (line) => {
    expect(text).toContain(line);
  });

  it.each(["What to see", "to write", "F36.AC", "Delete", "handle"])(
    "holds no draft line nor what F36 left to F37: %s",
    (word) => {
      expect(text).not.toContain(word);
    },
  );
});
