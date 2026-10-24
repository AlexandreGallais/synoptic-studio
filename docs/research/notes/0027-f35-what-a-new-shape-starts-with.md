# 0027 — Sources of F35: what a new shape starts with

Spike SP-026, 2026-10-22, for US-063's T3. When a tool places a shape, its box comes from the click (`interaction.md` §1); this note settles where its other settings come from — a corner radius, a polygon's corners and anchor, a text's font and size — and why. Every page cited was read through the local library the same day.

## 1. What each editor gives a new shape

| Editor | A new rectangle or polygon | A new text | A repeat of alike shapes | Sources |
| --- | --- | --- | --- | --- |
| Figma | fixed defaults: a rectangle 100 × 100 with a default fill, a polygon a triangle, a star five points | the last text style used, else Figma's default font; no setting chooses it | duplicating, components and styles | `REF-FIGMA-SHAPE-TOOLS`, `REF-FIGMA-CREATE-RECTANGLE`, `REF-FIGMA-TEXT-GUIDE`, `REF-FIGMA-FORUM-TEXT-DEFAULT` (level 6) |
| Inkscape | the last values of its tool: a value changed in the tool's bar, or read from a shape selected, is used for the next shape of that tool; a button resets them | the last style used, Inkscape's default since 1.2 for the Pen, Pencil, Text and Tweak tools | the tool's memory itself | `REF-INKSCAPE-TUTORIAL-SHAPES`, `REF-INKSCAPE-1-2-TOOL-STYLE` |
| Sketch | a fixed default style, which the designer replaces by a layer's style on demand (Set as Default Style) | the same | the default style set from a layer | `REF-SKETCH-DEFAULT-STYLE` |
| Affinity Designer | fixed defaults, taken from a selected object on demand (Synchronize defaults from selection), for the document or saved for all | the same | the defaults taken from an object | `REF-AFFINITY-DEFAULTS` |
| Penpot | not stated by its help | not stated | duplicating, and copying and pasting properties | `REF-PENPOT-LAYERS` (partial) |
| Illustrator | not read: Adobe's help answers 403 to the library and to WebFetch; its forums alone say a star keeps the previous count of points, level 6 and not cited | — | — | none |

Three ways come out:

- **Fixed defaults** (Figma's shapes): every new shape is the same, whatever came before; predictable, and alike shapes come from duplicating, components and styles.
- **The last values used** (Inkscape's shapes and texts, Figma's texts): what the designer just set is what the next shape gets; alike shapes need no other feature, but a value left from an earlier shape may surprise, hence Inkscape's reset button.
- **A default the designer sets** (Sketch, Affinity): fixed until the designer makes a shape's settings the default, by a command.

The sources agree on text, two independent ones (Inkscape's release notes, level 4, and Figma's forum, level 6, pointing the same way): a new text takes the last style used. On shapes they split: Figma fixed, Inkscape the last values.

## 2. The choice for F35 (US-063)

The agent's choice, the Product Owner having left it to research (2026-10-22), to confirm at VAL-036: **a new shape takes the settings of the last shape of its kind the panel showed**, Inkscape's way, the page's starting values before any; its box, place and angle come from the click, as Figma's do. The rule and its example are in `interaction.md` §1, What a new shape starts with.

Reasons:

1. **Figma's fixed defaults lean on features E02 lacks.** Figma gets alike shapes by duplicating, components and styles; duplicating is for later (`interaction.md` §1) and components and styles are other epics. With fixed defaults a symbol of three pentagons rounded to 8 would need its corners and radius typed three times.
2. **The panel already shows the values.** The playground's panel shows the settings of the shape edited; placing the next one with them makes what is shown what is placed, as Inkscape's tool bar does.
3. **Text agrees in both sources**: Figma and Inkscape both give a new text the last style used.
4. **One memory per kind**, as Inkscape keeps one per tool: a rectangle's radius set to 0 does not flatten the next polygon.

Set aside: Figma's fixed defaults for the reason 1; a default set by a command (Sketch, Affinity), a command more for the same result while the panel shows the values anyway. A reset to the page's starting values, Inkscape's button, is left for later: the Product Owner may ask for it at VAL-036.

## 3. What remains open

- The text's content: Figma and Inkscape start an empty text typed on the drawing, which the playground cannot do; the agent's choice is the content `Text`, selected in the Content field so that typing replaces it, to confirm at VAL-036.
- Illustrator's starting values, unread (above): not needed by the choice, two independent readable sources standing for each way.
