# DERIV-font-embedding — A font file embedded in an SVG: its permission and its length

Used by: EN-039 (whether a font may be embedded), EN-040 (the font written as a `data:` URL in base64). Sources: `REF-OT-OS2` (`fsType`, its bits and its version differences), `DERIV-bit-of-integer` (a bit of an integer, through `isBitSet`), `REF-RFC-4648` (base 64), `REF-RFC-2397` (the `data:` URL), `REF-CSS-FONTS-4` (§11.2, the media types and format hints). No single source states the rule of step 3 nor the length of step 6; each step cites what it rests on.

## Notation

| Symbol     | Definition                                                                         |
| ---------- | ---------------------------------------------------------------------------------- |
| `f`        | the `fsType` field of a font's `OS/2` table, an integer from 0 to 65 535           |
| `v`        | the version of that `OS/2` table, an integer ≥ 0                                   |
| `bit(i)`   | `isBitSet(f, i)`, bit `i` of `f`, 0 the least significant (`DERIV-bit-of-integer`) |
| `n`        | the length of a font file, in bytes                                                |
| `b₀ b₁ b₂` | three bytes of the file, each an integer from 0 to 255                             |

## Step 1 — What `fsType` says

`REF-OT-OS2` (`fsType`): bits 0 to 3 are the usage permissions, valid values 0, 2, 4 and 8:

| Value | Permission         | Embedding                                                                                               |
| ----- | ------------------ | ------------------------------------------------------------------------------------------------------- |
| 0     | installable        | may be embedded, and installed on the remote system                                                     |
| 2     | restricted licence | "must not be modified, embedded or exchanged in any manner without first obtaining explicit permission" |
| 4     | preview and print  | may be embedded and loaded temporarily "for purposes of viewing or printing"; the document read-only    |
| 8     | editable           | may be embedded and loaded temporarily; editing permitted                                               |

Bit 0 is "permanently reserved and must be zero"; bit 8, "No subsetting", forbids subsetting before embedding; bit 9, "Bitmap embedding only", forbids embedding any outline: a font without bitmaps "is considered unembeddable". Bits 4 to 7 and 10 to 15 are reserved. "Applications that implement support for font embedding must not embed fonts which are not licensed to permit embedding."

## Step 2 — What each version of the table allows

`REF-OT-OS2` (version differences):

- versions 0 and 1 assigned bits 0 to 3 only: "Applications must ignore bits 4 to 15 when reading a version 0 or version 1 table" — bit 9 is read from version 2;
- versions 0 to 2 did not make bits 0 to 3 exclusive: "the least-restrictive permission indicated take precedence" (2 and 8 set gives editable);
- from version 3 they must be exclusive; a font that sets several anyway: "some applications could assume the least-restrictive permission indicated".

## Step 3 — The rule: may this file be embedded whole?

The library embeds the whole file (no subsetting, outlines included). From steps 1 and 2:

```text
bitmapOnly  = v ≥ 2 and bit(9)
permits     = bit(2) or bit(3)                      preview and print, or editable: the least restrictive wins
restricted  = bit(1) and not permits
embeddable  = not bitmapOnly and not restricted
```

`REF-OT-OS2`: "For Restricted License embedding to take effect, the Embedding permissions sub-field must have the value 2 (that is, only bit 1 is set)"; with bit 2 or 3 also set, the permission is no longer restricted, as `permits` reads it. Bits 1, 2 and 3 all clear is installable (value 0), embeddable; bit 0 is not read, being reserved. Bit 8 does not forbid it: the whole file is not a subset. Taking the least restrictive permission for a table of version 3 or later that sets several bits follows the specification's note on what "some applications could assume" (provisional, VAL-017).

## Step 4 — Three bytes, four characters

`REF-RFC-4648`, §4: a 24-bit group, three bytes "concatenated", is "treated as 4 concatenated 6-bit groups", each an index into the alphabet `A`–`Z` (0–25), `a`–`z` (26–51), `0`–`9` (52–61), `+` (62), `/` (63). With `b₀` the most significant byte, the 24-bit number is `b₀ · 2¹⁶ + b₁ · 2⁸ + b₂`, and its 6-bit groups, by the division of `DERIV-bit-of-integer` step 2 applied to groups of 6 bits:

```text
c₁ = ⌊b₀ / 4⌋
c₂ = (b₀ mod 4) · 16 + ⌊b₁ / 16⌋
c₃ = (b₁ mod 16) · 4 + ⌊b₂ / 64⌋
c₄ = b₂ mod 64
```

`b₀ = 4c₁ + (b₀ mod 4)`: the top 6 bits of `b₀` are `c₁`; its 2 lowest bits are the top of `c₂`, above the top 4 bits of `b₁`; the lowest 4 bits of `b₁` are the top of `c₃`, above the top 2 bits of `b₂`; its lowest 6 bits are `c₄`.

## Step 5 — The last group

`REF-RFC-4648`, §4: when fewer than 24 bits remain, zero bits are added "to form an integral number of 6-bit groups", and `=` pads the group to 4 characters. One byte left gives `c₁ c₂` from `b₀` and `b₁ = 0`, then `==`; two bytes give `c₁ c₂ c₃` with `b₂ = 0`, then `=`.

## Step 6 — The length

Each full group of 3 bytes gives 4 characters (step 4), and a last group of 1 or 2 bytes gives 4 characters too (step 5), so a file of `n` bytes gives

```text
base64Length(n) = 4 · ⌈n / 3⌉
```

characters. The `data:` URL is `data:` and the media type, `;base64,` and those characters (`REF-RFC-2397`, §3). With the media types of `REF-CSS-FONTS-4`, §11.2 — `font/ttf` for a TrueType file and, the agent's choice among the media types §11.2 gives OpenType, `font/otf` for a file with CFF outlines — the prefix `data:font/ttf;base64,` or `data:font/otf;base64,` is 21 characters:

```text
dataUrlLength(n) = 21 + 4 · ⌈n / 3⌉
```

A file grows by a third: 4 characters for 3 bytes.

## Step 7 — Once per document

A face is one family name, one weight, one style and one file (`REF-CSS-FONTS-4`, §4.1: several `@font-face` rules build a family). A document (a symbol; a synoptic view with E08) that holds `k` texts in `m` distinct faces writes `m` rules, not `k`: its length is the sum of `dataUrlLength` over its distinct faces, whatever `k`.

## Cross-check

`REF-RFC-4648`, §10: `"foo"` is the bytes 102, 111, 111: `c₁ = ⌊102 / 4⌋ = 25` (`Z`), `c₂ = 2 · 16 + ⌊111 / 16⌋ = 32 + 6 = 38` (`m`), `c₃ = 15 · 4 + ⌊111 / 64⌋ = 60 + 1 = 61` (`9`), `c₄ = 111 mod 64 = 47` (`v`): `Zm9v`, as the test vector says. `"f"`: `c₁ = 25` (`Z`), `c₂ = 2 · 16 + 0 = 32` (`g`), then `==`: `Zg==`.

## Checks (test cases)

| Case                                                  | Computation                              | Expected                                 |
| ----------------------------------------------------- | ---------------------------------------- | ---------------------------------------- |
| Arial 5.01 (`fsType` 8, `OS/2` version 3)             | `bit(3)`: permits                        | embeddable                               |
| Inter 4.1 (`fsType` 0)                                | no bit set                               | embeddable                               |
| `fsType` 2                                            | `bit(1)`, no `bit(2)` nor `bit(3)`       | not embeddable                           |
| `fsType` 4                                            | `bit(2)`                                 | embeddable                               |
| `fsType` 10 (2 and 8), version 2                      | `bit(3)` wins                            | embeddable                               |
| `fsType` 0x0100 (no subsetting)                       | bit 8 does not forbid the whole file     | embeddable                               |
| `fsType` 0x0208 (bitmap only, editable), version 4    | `v ≥ 2` and `bit(9)`                     | not embeddable                           |
| `fsType` 0x0208, version 1                            | bit 9 ignored, `bit(3)`                  | embeddable                               |
| `fsType` 1 (reserved bit 0)                           | bits 1 to 3 clear: installable           | embeddable                               |
| `""`, `"f"`, `"fo"`, `"foo"`, `"foobar"`              | `REF-RFC-4648` §10                       | `""`, `Zg==`, `Zm8=`, `Zm9v`, `Zm9vYmFy` |
| `n` = 0, 1, 2, 3, 4                                   | `4 · ⌈n / 3⌉`                            | 0, 4, 4, 4, 8                            |
| Arial 5.01, 773 236 bytes                             | `⌈773 236 / 3⌉ = 257 746`, `× 4`, `+ 21` | 1 030 984 characters, URL 1 031 005      |
| a document of 10 texts, all in Arial regular and bold | 2 distinct faces                         | 2 rules                                  |
