# ADR-0034 — Only the coordinate typed is an integer; edits keep the fractions

**Status**: Accepted (Product Owner, VAL-005, 2026-10-11, Q28) — partly supersedes ADR-0031 (a vertex typed is an integer as a whole) and ADR-0003 (fractions only for a converted polygon); partly superseded by ADR-0035 (on a turned shape, the integer typed holds in the drawing only).

## Context

ADR-0031 says a vertex typed or moved by the user is an integer. ADR-0033 and US-014 type one coordinate at a time: typing x on a converted point keeps its fractional y. A free shape also keeps fractions after a stretch by width and height, a point added in the middle of an edge, or a deletion that shrinks its box: ADR-0003 allows them only for the vertices of a converted polygon. The audit of F08 (AUD-005) found the contradiction.

The Product Owner, testing F08: « si le x, je le bouge, lui, il a des virgules. Qu'est-ce qui se passe ? Il reste à la virgule. C'est trop de la balle. J'adore, franchement, j'adore. C'est non destructif. » — and on moving a point after a stretch: « ça va mettre à jour tous les points. C'est génial, c'est exactement ça que je voulais. »

## Decision

- A coordinate typed is an integer; the other coordinate of the point keeps its value, fractional or not.
- A free shape keeps exact numbers through every edit: its points and its size may be fractional after a conversion, a stretch, a point added in the middle of an edge or a deletion. Nothing is rounded in the model; the user types integers only.
- Editing a point of a stretched free shape works on its points as drawn, so every point is updated to its drawn place (ADR-0033).

## Consequences

- ADR-0031's "a vertex typed or moved by the user is an integer" now reads per coordinate; its other decisions stand.
- ADR-0003's integer model holds for what the user types; the stored free shape may hold fractions from any edit. The output keeps at most 5 decimals (Q10).

## References

ADR-0003, ADR-0031, ADR-0033, Q27, Q28
