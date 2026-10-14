# DERIV-bit-of-integer — A bit of a non-negative integer, by division

Used by: EN-031 (`isBitSet`, reading the flags of a font's tables). Sources: `REF-WIKI-BIT-NUMBERING` (an unsigned integer is the sum of its bits times powers of two, bits numbered from 0 at the least significant), `REF-MDN-REMAINDER` (the remainder of a division, through `modulo`). The bitwise operators of ECMAScript work on 32-bit signed integers; this reading works on every safe integer.

## Notation

| Symbol | Definition                                                          |
| ------ | ------------------------------------------------------------------- |
| `n`    | an integer ≥ 0                                                      |
| `bₖ`   | the bit of number `k` of `n`, 0 or 1, `k = 0` the least significant |
| `i`    | the number of the bit read, an integer ≥ 0                          |

## Step 1 — The integer as its bits

In LSb 0 numbering, the value of an unsigned binary integer is `n = Σₖ bₖ · 2ᵏ` (`REF-WIKI-BIT-NUMBERING`).

## Step 2 — Dividing by 2ⁱ

Divide by `2ⁱ` and split the sum at `i`:

```text
n / 2ⁱ = Σ_{k ≥ i} bₖ · 2^(k − i)  +  Σ_{k < i} bₖ · 2^(k − i)
```

The first sum is an integer. The second is at most `Σ_{k < i} 2^(k − i) = 1 − 2^(−i) < 1` and at least 0. So the floor of `n / 2ⁱ` is the first sum alone: `⌊n / 2ⁱ⌋ = bᵢ + Σ_{k > i} bₖ · 2^(k − i)`.

## Step 3 — The remainder by 2

Every term of `Σ_{k > i} bₖ · 2^(k − i)` is a multiple of 2, so the remainder of `⌊n / 2ⁱ⌋` divided by 2 is `bᵢ` (`REF-MDN-REMAINDER`, the sign of the dividend positive):

```text
bᵢ = ⌊n / 2ⁱ⌋ mod 2
```

## Cross-check

149 is `1001 0101` (`REF-WIKI-BIT-NUMBERING`'s example): `⌊149 / 1⌋ mod 2 = 1`, `⌊149 / 2⌋ = 74`, `74 mod 2 = 0`, `⌊149 / 4⌋ = 37`, `37 mod 2 = 1`, `⌊149 / 128⌋ = 1`, `1 mod 2 = 1`: bits 0, 2 and 7 are 1, bit 1 is 0.

## Checks (test cases)

| Case            | Computation             | Expected |
| --------------- | ----------------------- | -------- |
| 149, bit 0      | `149 mod 2`             | 1        |
| 149, bit 1      | `74 mod 2`              | 0        |
| 0xFFFF, bit 15  | `⌊65 535 / 32 768⌋ = 1` | 1        |
| 0xFFFF, bit 16  | `⌊65 535 / 65 536⌋ = 0` | 0        |
| 0x8000, bit 15  | `⌊32 768 / 32 768⌋ = 1` | 1        |
| 0x8000, bit 14  | `⌊32 768 / 16 384⌋ = 2` | 0        |
| 2³² + 1, bit 32 | `⌊(2³² + 1) / 2³²⌋ = 1` | 1        |
