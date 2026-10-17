---
id: US-062
epic: E17
feature: F18
title: Installed fonts read whole before they are listed
status: ready
points: 3
---

# E17 · F18 · US-062 — Installed fonts read whole before they are listed

As a symbol designer, I want an installed font listed only once each of its files has been read whole, so that the list never offers a font that cannot be drawn or embedded.

Decided by the Product Owner at VAL-019 (2026-10-17): the list holds only fonts known to be sound. Until now the first reading of the installed fonts kept each face's metrics, and a face could still be refused when its family was chosen and its files read again (US-053, US-060, US-061).

## Acceptance criteria

- Given the installed fonts being read, when a face's file cannot be read whole, then that face is not listed, and a family left with no face is not listed, the counting line saying how many were refused.
- Given a face's file read whole at the first reading, when it has been checked, then its bytes are released at once and only its metrics are kept, so that the memory held stays that of one file at a time (a 191 MB file is read, checked and released).
- Given a family chosen from the list, when its files are read again, then the reading still happens and a refusal at that moment, possible only for a font removed meanwhile, is handled as before (US-061) and said.
- Given the first reading timed on this computer before and after, when the story ends, then both durations are written in the story and the domain.

## Tasks

One task = one commit, referenced as `US-062.Tn`.

- [ ] T1 — Each face read whole and checked at the first reading, its bytes released; tests (2 h)
- [ ] T2 — The counting line and the domain; durations measured; Product Owner test card (1 h)
