# DERIV-kerning-classes — The kerning of characters kept by classes of characters that kern alike

Used by: EN-031 (`kerningClasses`, `classesByCharacter`, `kerningOfCharacters`, `pairKerning`, ADR-0052). Sources: `REF-WIKI-EQUIVALENCE-CLASS` (the equivalence classes of the kernel of a function form a partition), `REF-MATHWORLD-TRANSPOSE` (the columns of a matrix are the rows of its transpose), `REF-OT-GPOS` (pair adjustment format 2 represents pairs "as sequences of two class values" in a two-dimensional array).

The kerning of `n` characters with each other is an `n × n` table. Read once from the font, it is kept smaller: characters that kern alike share a class, and a pair's kerning is read from a table of classes. This derivation shows the smaller table gives every pair the same kerning as the full one.

## Notation

| Symbol         | Definition                                                                   |
| -------------- | ---------------------------------------------------------------------------- |
| `C`            | the characters kept, `n` of them                                             |
| `K(a, b)`      | the kerning of `a` then `b`, font units, read in the font's lookups          |
| `row(a)`       | the kerning of `a` as the first character: `(K(a, c))` for every `c` of `C`  |
| `column(b)`    | the kerning of `b` as the second character: `(K(c, b))` for every `c` of `C` |
| `L(a)`, `R(b)` | the left class of `a`, the right class of `b`                                |
| `V(l, r)`      | the kerning kept for left class `l` and right class `r`                      |

## Step 1 — Classes of characters that kern alike

Let `a ∼ a′` when `row(a) = row(a′)`. It is the kernel of the function `row`: "x₁ ∼ x₂ if and only if f(x₁) = f(x₂)", and its equivalence classes form a partition of `C`, "every element of the set belongs to exactly one equivalence class" (`REF-WIKI-EQUIVALENCE-CLASS`). Number the classes: class 0 for the rows of zeros, then 1, 2, … in the order they first appear. That is `L`. The same with `column` gives `R`; the columns of the table are the rows of its transpose (`REF-MATHWORLD-TRANSPOSE`).

## Step 2 — The table of classes

For each left class `l`, take one character `a_l` of it; for each right class `r`, one character `b_r`. Let `V(l, r) = K(a_l, b_r)`. For class 0, whose rows (or columns) are zeros, `V` is 0, and no character needs to be taken.

## Step 3 — Every pair keeps its kerning

Take any `a` and `b` of `C`, `l = L(a)`, `r = R(b)`.

- `a` and `a_l` are in the same class, so `row(a) = row(a_l)` (step 1): `K(a_l, c) = K(a, c)` for every `c`, and so `K(a_l, b_r) = K(a, b_r)`.
- `b` and `b_r` are in the same class, so `column(b) = column(b_r)`: `K(c, b_r) = K(c, b)` for every `c`, and so `K(a, b_r) = K(a, b)`.

Therefore `V(L(a), R(b)) = K(a_l, b_r) = K(a, b)`. For class 0 the same holds: a row of zeros, or a column of zeros, gives `K(a, b) = 0 = V`.

## Step 4 — What is stored

Only the classes other than 0 are stored, by code point; a character left out is in class 0. A pair's kerning is then `V(L(a) or 0, R(b) or 0)`: two look-ups by code point and one in the table, whatever the number of characters. Pair adjustment format 2 stores kerning in the same way, by classes of glyphs (`REF-OT-GPOS`); these classes are of characters, and built from the kerning read rather than copied from the font.

## Cross-check

A, V, T, o with A V −80, V A −80, A T −40, T A −40, T o −60: rows A `(0, −80, −40, 0)`, V `(−80, 0, 0, 0)`, T `(−40, 0, 0, −60)`, o zeros give left classes 1, 2, 3, 0; columns A `(0, −80, −40, 0)`, V `(−80, 0, 0, 0)`, T `(−40, 0, 0, 0)`, o `(0, 0, −60, 0)` give right classes 1, 2, 3, 4. `V(3, 4) = K(T, o) = −60`; `V(0, r) = 0` for o first.

## Checks (test cases)

| Case                                                    | Expected                                 |
| ------------------------------------------------------- | ---------------------------------------- |
| rows `(0, 0)`, `(−1, 2)`, `(0, 0)`, `(3, 0)`, `(−1, 2)` | classes 0, 1, 0, 2, 1                    |
| rows `(1, 2)`, `(2, 1)`: equal sums, rows differ        | classes 1, 2                             |
| the cross-check, T then o                               | −60                                      |
| no kerning at all                                       | every character in class 0, `V = [[0]]`  |
| random pairs of 6 glyphs                                | `V(L(a), R(b)) = K(a, b)` for every pair |
