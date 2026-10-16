---
id: US-061
epic: E17
feature: F18
title: Font list held while the default font is read again
status: ready
points: 2
---

# E17 · F18 · US-061 — Font list held while the default font is read again

As a symbol designer, I want the font list held while the default font is read again, and a face refused at that reading no longer measured, so that I never choose at the wrong moment nor measure a face the browser no longer gives (left by US-060: I77, I78; F18.AC14).

## Acceptance criteria

- Given the default font's files read again, when the reading is not finished, then the font list is disabled and says it is reading.
- Given a face refused at the second reading of its family, when a text uses that family, then the face's metrics read ahead are dropped and the text is measured as for a face not held, or the refusal is said.

## Product Owner test

| Step | Do                                                                                                  | You should see                                                        |
| ---- | --------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| 1    | open the playground, choose the Text tool, and at once open the font list while Inter is read again | the list is greyed, with a line saying the default font is being read |
| 2    | wait for the reading to end                                                                         | the list is enabled; Inter is chosen                                  |

## Tasks

One task = one commit, referenced as `US-061.Tn`.

- [ ] T1 — The list disabled during the second reading; tests (1 h)
- [ ] T2 — A face refused at the second reading dropped; tests and playground (1 h)
