# F16 — Colors and their picker

## What is new

A color is now typed or pasted in any common hexadecimal form, or picked with a **picker**: a swatch beside the field opens a plane, a hue bar and an opacity bar, with the values in HSB or HSL. The text's color and its background's both have one. What is stored stays the hexadecimal and the opacity, so a symbol's file never changes with the way its color was chosen.

![“Pump P-101” in #ff8000; in #0000ff at 80 %; in #1e293b on a #22c55e band at 40 %](./images/f16-colors.svg)

| Part                                    | What it does                                                                                                        |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| **Color** field                         | 3, 4, 6 or 8 hexadecimal digits, with or without `#`, in any case; kept as `#` and six lower-case digits            |
| **Opacity** field                       | an integer from 0 to 100 %; 4 or 8 digits typed set it from their alpha                                             |
| **Swatch**                              | the square left of the field, in the color; a click opens the picker under the color's fields, another click closes |
| **Plane**                               | the saturation across, 0 at the left; the brightness (HSB) or the lightness (HSL) up, 100 at the top                |
| **Hue bar**                             | the hue from 0 to 359°, whole degrees                                                                               |
| **Opacity bar**                         | the opacity from 0 to 100 %, a whole percent                                                                        |
| **Model**, **H**, **S**, **B** or **L** | HSB or HSL, and their three values as whole numbers, typed or shown                                                 |

Rules: [domain, "Colors"](../domain/interaction.md#_5-colors-align-and-distribute) and Q38; formulas: [`DERIV-color-spaces`](../derivations/color-spaces.md); decision: [ADR-0056](../adr/0056-color-stored-as-hexadecimal-hsl-and-hsb-as-views.md).

## The ideas behind it

| Term                  | In plain words                                                                                           |
| --------------------- | -------------------------------------------------------------------------------------------------------- |
| **Hexadecimal color** | red, green and blue, each from 0 to 255 written in two digits from 00 to ff: `#ff8000` is orange         |
| **Alpha**             | a fourth pair of digits for the opacity: `cc` is 204 of 255, 80 %                                        |
| **Hue**               | the color's place on the color wheel, in degrees: 0 red, 120 green, 240 blue                             |
| **Saturation**        | how far from grey: 0 % is a grey                                                                         |
| **Brightness** (HSB)  | how far from black: 100 % is the pure hue or white                                                       |
| **Lightness** (HSL)   | from black to white, 50 % being the pure hue                                                             |
| **Slider**            | a bar with a thumb, moved by mouse or keys: arrows 1, Page Up and Page Down 10, Home and End to the ends |

**Short forms.** `f80` doubles each digit: `#ff8800`. `#0000ffcc` keeps `#0000ff` and sets the opacity to 80 %. Anything else, `red` or `#ff800`, is refused: the field shows its previous color again, a line under it says why.

**HSB and HSL are views.** They are computed from the hexadecimal each time the picker opens, never stored. `#ff8000` is HSB 30°, 100 %, 100 % and HSL 30°, 100 %, 50 %, whole numbers, the decimals kept inside (ADR-0058). Editing one value keeps the exact others: `#336699` is HSB 210°, 67 %, 60 %; its brightness set to 61 gives `#34689c`, where the three whole numbers would give `#33679c`, and 67 left as shown keeps `#336699` (ADR-0059).

**The plane**, described rather than drawn: in HSB, white at the top left, the pure hue at the top right, black along the bottom; in HSL, white along the top, black along the bottom, grey at the left and the pure hue in the middle of the right edge. At the hue 210°, the middle of the right edge in HSB is `#004080`.

**A grey has no hue.** `#808080` opens at 0°; while the picker stays open it keeps the hue its bar shows, so dragging towards more saturation gives a blue at 200°, not a red.

## How to try it

`npm run dev` and `http://localhost:5173`: 40 steps, each one line to do and one to look at; **Next** types the text and its colors. At the epic review they come back as steps 162 to 201 of `?guide=epic`. Next does not choose the picker's model: a step that needs one names it.

| Steps | What                                                                                                    |
| ----- | ------------------------------------------------------------------------------------------------------- |
| 1–7   | six digits, with `#` and spaces, three, four and eight digits, a color pasted, a value refused          |
| 8–10  | the opacity, the swatch opening the picker, the picker closed by the swatch, Escape and a click outside |
| 11–13 | the hue bar by mouse and by keys, the opacity bar                                                       |
| 14–17 | the plane in HSB and in HSL, values typed, a grey keeping its hue                                       |
| 18    | the background's color and its picker                                                                   |
| 19–38 | the 20 decisions below, one step each with its example                                                  |
| 39–40 | back to F14 and F05, then to the rectangle, drawn as before                                             |

## Decided by the agent, to confirm

Each is built as written and shown by its step; the full list, with its examples, is in [VAL-016](../backlog/stories/E15-F16-VAL-016-validate-f16.md#decisions-to-confirm).

| #   | Question                       | As built                                                                                  | Step |
| --- | ------------------------------ | ----------------------------------------------------------------------------------------- | ---- |
| 1   | What is stored                 | the hexadecimal and the opacity; HSL and HSB computed, never stored                       | 19   |
| 2   | Forms typed                    | with or without `#`, any case, spaces around ignored; kept as six lower-case digits       | 20   |
| 3   | Short forms and alpha          | 3 digits doubled; 4 and 8 digits set the opacity to the nearest percent (`ec` gives 93 %) | 21   |
| 4   | Anything else; a copy          | refused, the previous color back; a copy is six digits, the opacity apart                 | 22   |
| 5   | Opening and closing            | the swatch opens; the swatch, Escape or a click outside closes                            | 23   |
| 6   | First model                    | HSB, the agent's own choice; HSL one choice away                                          | 24   |
| 7   | The plane's axes               | saturation across, 0 at the left; brightness or lightness up, 100 at the top              | 25   |
| 8   | Decimals shown                 | one, so that a value typed back gives the same color                                      | 26   |
| 9   | Rounding to the hexadecimal    | each of red, green and blue to the nearest, a tie up                                      | 27   |
| 10  | A grey's hue                   | 0° shown; an open picker keeps its bar's hue                                              | 28   |
| 11  | Keys                           | arrows 1, Page Up and Down 10, Home and End; on the plane Left and Right the saturation   | 29   |
| 12  | The hue bar                    | to one decimal, arrows add 1, End 359.9°, 360 shown 0                                     | 30   |
| 13  | The background's color         | read into F16's fourth criterion: the same swatch and picker                              | 31   |
| 14  | A color refused                | the color the field had when it got the focus, a line under it                            | 32   |
| 15  | Beyond the ends; empty         | held at the end; an empty value shown again; a click on the opacity bar a whole percent   | 33   |
| 16  | Where the picker opens; Escape | under its color's fields, which stay usable; Escape gives the focus back to the swatch    | 34   |
| 17  | Another model chosen           | keeps the hue held while it gives the color back, else the color's own hue                | 35   |
| 18  | What a screen reader reads     | each bar's track is the slider, with its name and value; the thumb only drawn             | 36   |
| 19  | A value on a half-tenth        | may be shown rounded down, either neighbour giving the color back                         | 37   |
| 20  | Page keys on the plane         | Page Up, Page Down, Home and End act on the saturation                                    | 38   |

## Limits

- Colors of the text and of its background only: shapes have no color field yet.
- No saved colors yet: with the epic of libraries (F16, "Left for later"); no named colors such as `red`.
- One opacity per color: the alpha typed in 4 or 8 digits goes to the Opacity field, never into the hexadecimal stored.
- The plane and the bars are drawn by the page's styles, outside the symbol: they are not exported.

## Validation

To be run by the Product Owner (`VAL-016`, T3).

| Remark | What was done or where it went |
| ------ | ------------------------------ |
