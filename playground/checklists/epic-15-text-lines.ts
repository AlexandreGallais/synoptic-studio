import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E15's checklist on the text and its box (US-051): F05's options, then F14's, each
 * example in Inter 16 or 32 as the demo pages give it.
 */
export const EPIC_15_TEXT_LINES: readonly ChecklistLine[] = [
  {
    feature: "F05",
    todo: "Choose the Text tool, Inter 4.1 installed",
    see: "“Pump P-101” over “Running” in Inter 32, the dashed frame hugging the letters",
  },
  {
    feature: "F05",
    todo: "Add a line with Enter, then empty the content",
    see: "three lines, the frame taller; then nothing drawn, no frame, no message",
  },
  {
    feature: "F05",
    todo: "Size 48, then 16",
    see: "the text and its frame grow, then shrink, together",
  },
  {
    feature: "F05",
    todo: "Weight 100, press Bold, press it again; then weight 900",
    see: "thin, then 700, then thin again; then black",
  },
  {
    feature: "F05",
    todo: "Italic on, then off",
    see: "the letters slanted, then upright",
  },
  {
    feature: "F05",
    todo: "Color #ff0000, opacity 40, then 0",
    see: "red, see-through; at 0 the text goes, the frame stays",
  },
  {
    feature: "F05",
    todo: "Letter spacing 10, then −2; word spacing 20",
    see: "the letters spread, nothing added after the last, then tighter; only the spaces widen",
  },
  {
    feature: "F05",
    todo: "Underline on, then Strike-through on",
    see: "one stroke under each line, as long as it, then one through it, in the text's color",
  },
  {
    feature: "F05",
    todo: "Choose each Case, the content “pump P-101 is running”",
    see: "the drawing in that case (camelCase pumpP101IsRunning, kebab-case pump-p-101-is-running…), the field as typed",
  },
  {
    feature: "F05",
    todo: "Alignment Center, then Right",
    see: "“Running” moves to the middle, then to the right of the block; “Pump P-101” stays",
  },
  {
    feature: "F05",
    todo: "Direction Right to left, the content a Hebrew word",
    see: "the letters run from the right; in Inter, which has no Hebrew, missing-glyph boxes",
  },
  {
    feature: "F05",
    todo: "X 100, Y 50, then Angle 90, 370 and −90",
    see: "text and frame move together; a quarter turn about the frame's middle, strokes turning too; 370 kept as 10, −90 as 270",
  },
  {
    feature: "F05",
    todo: "Size 0, then back to 32",
    see: "refused: the drawing gone, a message, the field in red; back when valid",
  },
  {
    feature: "F05",
    todo: "Turn the text, choose the rectangle, then the text again",
    see: "each tool keeps its own X, Y and angle; the rectangle 120 × 80 as before",
  },
  {
    feature: "F14",
    todo: "Fit Box, width 50, height 100, the content “Hello World”, Inter 16",
    see: "the box's fields shown; “Hello” over “World”; the dashed frame is the box",
  },
  {
    feature: "F14",
    todo: "The content “Pump P-101 is running” in that box",
    see: "four lines, Pump, P-101, is, “runn…”: “running” never broken",
  },
  {
    feature: "F14",
    todo: "Ellipsis off, then on",
    see: "“running” whole, past the box's right edge; then “runn…” again",
  },
  {
    feature: "F14",
    todo: "Max lines 1, then 0",
    see: "one line ending with “…”; then every line back",
  },
  {
    feature: "F14",
    todo: "The content “A”, “B”, an empty line, “C”, Max lines 3, then 0",
    see: "“A” over “B…”, no “…” alone on the empty line, which takes no height (US-058); then the four lines",
  },
  {
    feature: "F14",
    todo: "Vertical alignment Middle, then Bottom",
    see: "the block of lines moves down, the last line's bottom on the box's at Bottom",
  },
  {
    feature: "F14",
    todo: "Width 200, Alignment Center, then Right",
    see: "each line moves across the box, not across its own block",
  },
  {
    feature: "F14",
    todo: "Line height 24, then 0; Trim on",
    see: "baselines 24 apart, then the font's own; trimmed, the first capitals touch the box's top",
  },
  {
    feature: "F14",
    todo: "Background on, then its color #22c55e at 40 %",
    see: "a yellow opaque band behind each line, then light green; overlapping bands no darker",
  },
  {
    feature: "F14",
    todo: "“Hello World ” (a space at the end), width 50, Background on",
    see: "two bands, each as long as its word, “World”'s not past the box (US-049)",
  },
  {
    feature: "F14",
    todo: "Angle 90 on the text in its box",
    see: "text and box turn together about the box's middle",
  },
  {
    feature: "F14",
    todo: "Fit content again, then Box",
    see: "the box's fields go, the frame hugging the lines as in F05; then the box typed comes back, 50 × 100, not measured again (US-057)",
  },
  {
    feature: "F14",
    todo: "Box Reset",
    see: "the box takes back the size the lines fit; Fit content then Box measures them again (US-057)",
  },
];
