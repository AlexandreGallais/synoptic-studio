---
id: US-035
epic: E15
feature: F16
title: Type or pick a color and its opacity
status: done
points: 3
---

# E15 · F16 · US-035 — Type or pick a color and its opacity

As a designer, I want to type or paste a color with its opacity, both always shown, or pick it in HSL or HSB, so that I copy colors quickly and choose them the way that suits me.

The text's color and its background color use it in the playground; the shapes' fills and strokes will (E04). Rules: `interaction.md` §5 (Colors), ADR-0056, `DERIV-color-spaces`.

## Acceptance criteria

- Given a text, when the designer types `F80` in its color field, then the field shows `#ff8800` and the text is drawn with `fill="#ff8800"`; when they paste `#0000ffcc` in its background color field, then that field shows `#0000ff`, the background opacity 80, and the background is drawn with `fill-opacity="0.8"` (EN-037).
- Given `red` or `#ff800` typed in a color field, when it is left, then it shows its previous color and the page says why (EN-037).
- Given the color `#ff8000`, when the designer clicks its swatch, then a picker opens with a hue slider at 30.1°, an opacity slider, the plane in HSB, the values 30.1°, 100 %, 100 %, and the fields `#ff8000` and its opacity; when they choose HSL, then the values show 30.1°, 100 %, 50 % and the plane shows the lightness up (`REF-FIGMA-COLOR-PICKER`, `REF-INKSCAPE-1-2-COLOR`).
- Given the picker in HSB at hue 210°, when the designer clicks the middle of the plane's right edge, then the saturation is 100 %, the brightness 50 % and the color `#004080` (`DERIV-color-spaces`, step 10); when they type 66.7 in the saturation and 60 in the brightness, then the color is `#336699`.
- Given the hue slider focused at 30.1°, when the designer presses Right, Page Up, Home and End, then it shows 31.1°, 41.1°, 0° and 359.9°, a hue of 360 being shown 0.0, and the color follows; the opacity slider and the plane answer the same keys (`REF-WAI-APG-SLIDER`).
- Given the grey `#808080` and the hue slider at 200°, when the designer drags the plane to a saturation above 0, then the hue stays 200° while the picker is open.
- Given the picker open, when the designer presses Escape, clicks the swatch or clicks outside, then it closes and the text keeps only the hexadecimal and the opacity; given a color picked, then every shape and text of E01, E14, F05 and F14 draws as before.

## Product Owner test

Open `http://localhost:5173/`, click **Text** and click on the drawing (Inter 16, as the page opens).

| Step | Do                                                                            | You should see                                                             |
| ---- | ----------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 1    | Type `F80` in **Color**, press Tab                                            | `#ff8800` in the field, the text orange                                    |
| 2    | Turn **Background** on, paste `#0000ffcc` in its color, press Tab             | `#0000ff`, its opacity 80, a blue band a little transparent                |
| 3    | Type `red` in **Color**, press Tab                                            | the field back to `#ff8800`, a red line under it saying why                |
| 4    | Type `#ff8000` in **Color**, click its swatch                                 | the picker: hue at 30.1°, values 30.1°, 100 %, 100 %, plane in HSB         |
| 5    | Choose **HSL**                                                                | values 30.1°, 100 %, 50 %; the plane white at the top, black at the bottom |
| 6    | Choose **HSB**, type 210 in **H**, click the middle of the plane's right edge | `#004080`                                                                  |
| 7    | Type 66.7 in **S** and 60 in **B**, Tab after each                            | `#336699`                                                                  |
| 8    | Focus the hue slider, press Right, then Page Up, Home, End                    | 211°, 221°, 0°, 359.9°                                                     |
| 9    | Press Escape                                                                  | the picker closed, the color kept (`#993333` after End)                    |

## Notes

- Each slider's role `slider`, its name and its values sit on its track, the element that takes the focus and the keys, and the thumb is only drawn; the WAI-ARIA slider pattern puts them on the thumb (`REF-WAI-APG-SLIDER`). The plane does the same. A provisional choice, to confirm at VAL-016.
- The bytes of values picked or typed to one decimal are computed in exact integers, so that a byte lying on a half rounds up (`DERIV-color-spaces`, step 11; US-035 audit).

## Tasks

One task = one commit, referenced as `US-035.Tn`.

- [x] T1 — The color fields: a swatch beside the hexadecimal and opacity fields, a typed or pasted form read by the library, refused with a message, for the text's color and its background (2 h)
- [x] T2 — The picker: opening and closing, the hue and opacity sliders with their keys, the choice of HSL or HSB, the three values to one decimal, typed back through the library (3 h)
- [x] T3 — The plane drawn for the hue chosen from the library's conversion, picked by a click, a drag or the keyboard; a grey keeps the slider's hue while the picker is open (3 h)
- [x] T4 — Playground tests end to end of the steps above; the test card checked (2 h)
