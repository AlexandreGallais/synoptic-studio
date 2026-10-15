---
id: US-037
epic: E15
feature: F05
title: The bold button gives back the weight before, and sentence case removed
status: ready
points: 2
---

# E15 · F05 · US-037 — The bold button gives back the weight before, and sentence case removed

As a designer, I want the bold button to give back the weight my text had before I pressed it, and the case list to hold only the cases I use, so that a thin text made bold and back stays thin.

Product Owner decisions of 2026-10-15, from a try of the playground before VAL-014: VAL-014's decision 3 settled as its option 3; sentence case is of no use; title, camel, Pascal, kebab, snake, constant, upper and lower are kept. Rules: `shapes.md` (Text, Characters, Case transforms), Q36.

Built on the latest branch of the chain, `feature/f15-fonts-html`, after US-036.

## Acceptance criteria

- Given a text at weight 100 (Thin), when the designer clicks **Bold**, then the weight is 700 and the button shows pressed; when they click it again, then the weight is 100, not 400.
- Given a text at weight 800, when the designer clicks **Bold**, then the weight is 700; clicked again, 800.
- Given a text at weight 100 made bold (700), when the designer chooses 500 in the weight list, then clicks **Bold** twice, then the weight is 700, then 500: the weight chosen in the list replaced the one held.
- Given a text created at 700, no weight held, when the designer clicks **Bold**, then the weight is 400 (agent's reading, to confirm at VAL-014).
- Given the case list, when the designer opens it, then it offers none, upper, lower, title, camel, Pascal, kebab, snake and constant, and no sentence case; given a text read with the case `sentence`, then it is refused as any case outside its values (`isValidText`).
- Given "pump P-101 is running" in title case, then it reads `Pump P-101 Is Running`, as before (F05.AC4).

## Product Owner test

Open `http://localhost:5173/`, click **Text**, click on the drawing and type `pump P-101 is running`.

| Step | Do                                           | You should see                                      |
| ---- | -------------------------------------------- | --------------------------------------------------- |
| 1    | Choose the weight Thin (100), click **Bold** | the text bold, the weight 700, the button pressed   |
| 2    | Click **Bold** again                         | the text thin again, the weight 100                 |
| 3    | Choose 800, click **Bold** twice             | 700, then 800                                       |
| 4    | Open the case list                           | no sentence case; title, camel, Pascal… still there |
| 5    | Choose **Title**                             | `Pump P-101 Is Running`                             |

## Tasks

One task = one commit, referenced as `US-037.Tn`.

- [ ] T1 — The bold button holds the weight before it was pressed, gives it back on the second click, forgets it when a weight is chosen in the list, 400 when none is held; the playground's tests first (2 h)
- [ ] T2 — Sentence case removed from the model's cases, `textInCase`, the playground's list and the guided steps; a text read with `sentence` refused; tests first (2 h)
- [ ] T3 — The guided steps of F05 and VAL-014's decisions 1 and 3 brought up to date; the test card (1 h)
