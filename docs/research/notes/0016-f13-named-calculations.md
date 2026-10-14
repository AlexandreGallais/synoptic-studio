# 0016 — Sources of F13: the named calculations

Spike SP-014, 2026-10-14. Two math-expert agents read every non-test file of `src/` and `playground/` and listed each inline calculation that mathematics names, with a source and whether naming it keeps the order of floating-point operations; each "exact" claim was checked with `Object.is` over ±0, subnormals and large values. A golden file of 400 drawings, conversions and handle values, written before the passes, proves every result unchanged after them.

## Pass 1, `math` and `geometry` (EN-028)

| New function                                     | Replaces                                                                      | Source                                 |
| ------------------------------------------------ | ----------------------------------------------------------------------------- | -------------------------------------- |
| `distance(a, b)`                                 | `norm(subtract(a, b))`, four places                                           | `REF-MATHWORLD-DISTANCE`               |
| `rotate(vector, direction)`                      | the rotation inside `turnAbout`, and `turnAbout(v, 0, d)`                     | `REF-MATHWORLD-ROTATION-MATRIX`        |
| `hadamardProduct(vector, factors)`               | `stretchedHandles`, `stretchedToEllipse` (one duplicate gone)                 | `REF-WIKI-HADAMARD`                    |
| `opposite(vector)`                               | `scale(v, -1)`                                                                | `REF-MATHWORLD-VECTOR-DIFFERENCE`      |
| `quarterTurns(vector, count)`                    | the repeated quarter turn of `turnDirection`                                  | `DERIV-turned-shape`                   |
| `degreesToRadians`, `radiansToDegrees`           | the two conversions, and the two `HALF_TURN` constants                        | `REF-OPENSTAX-ANGLES`                  |
| `directionAt(angle)`                             | `{ x: cos a, y: sin a }`, four places                                         | `REF-OPENSTAX-UNIT-CIRCLE`             |
| `polarAngle(vector)`, `signedAngle(from, to)`    | `atan2(y, x)`; `atan2(perpDot, dot)` — the zero guard stays in `turningAngle` | `REF-MDN-ATAN2`, `DERIV-turning-angle` |
| `modulo(dividend, divisor)`                      | `((n % d) + d) % d` in `degreesInTurn` and `cyclicItem`                       | `REF-MDN-REMAINDER`                    |
| `average(a, b)`                                  | `(a + b) / 2` in the bisections                                               | `REF-MATHWORLD-MIDPOINT`               |
| `discriminant(quadratic)`                        | `b² − 4ac` in `quadraticRoots`                                                | `REF-WIKI-QUADRATIC`                   |
| `cubicBernstein(t)`, `cubicDerivativeWeights(t)` | the weights of `cubicPoint` and `cubicVelocity`                               | `REF-WIKI-BEZIER`                      |
| `boxSize(box)`                                   | `maxX − minX`, `maxY − minY`, five places                                     | `REF-WIKI-BOUNDING-BOX`                |
| `scalarProjection`                               | `dot(subtract(P, B), u)` beside a curve                                       | `REF-MATHWORLD-SCALAR-PROJECTION`      |
| `chainEnd(start, directions)`                    | the running sums of cosines and sines of the stretched polygon                | `REF-CRANE-DISCRETE-CURVES`            |
| `centralAngle(step, corners)`                    | `(2π · k) / n`                                                                | `DERIV-stretched-polygon`              |

Existing names used where the code wrote them out: `subtract`, `add`, `scale`, `ZERO_VECTOR`, `withoutNegativeZero`, `EPSILON` (a second constant of the same value).

## Pass 1, model, io, render and playground (EN-029)

`hasArea(size)` (`REF-WIKI-RECTANGLE`, `width · height > 0`), `halfSize(size)` (`DERIV-turned-shape`), `explement(angle)` (`REF-WIKI-ANGLE`), `clamp` (`REF-WIKI-CLAMP`), `turnAboutBy(point, center, angle)`; and calls to `boxCenter`, `add`, `subtract`, `boxSize`, `modulo` and `mirroredHandle` where the code wrote them out.

## Left inline, with the reason

| Calculation                                                | Reason                                                                                |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `d − floor(d/90)·90` in `turnDirection`                    | `modulo(d, 90)` differs when `d/90` rounds up                                         |
| `t − end / SAMPLES` in `crossings`                         | not always equal to the previous grid point                                           |
| the translation of `freeShapeContour`, `dx === 0 ? …`      | `add` turns −0 into +0                                                                |
| `(b < 0 ? −1 : 1)` in `quadraticRoots`                     | a sign with sign(0) = +1, not `Math.sign`; no standard name                           |
| `cubicVelocity` beside `derivativeCoefficients`            | Bernstein and power forms of one derivative round differently: documented, not merged |
| `midpoint` beside `pointBetween(…, 1/2)`                   | `(a + b)/2` and `a/2 + b/2` differ on overflow and subnormals: both kept              |
| the playground's handle bar and inset box                  | drawings of the playground only; a band and an inset need a derivation of their own   |
| one-sided tolerance tests (`radius − effective ≥ EPSILON`) | no standard name                                                                      |
| evenly spaced samples `(k · T) / n`                        | no source read for the name; pass 2                                                   |

## Open

A handle of length 0 shows a side angle of 180° when both components of its side's direction are negative (`atan2(+0, −0) = π`), 0° otherwise. F13 keeps it; a question for the Product Owner (Q35).
