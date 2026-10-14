# F13 — Every calculation named

## What changed

Nothing on the page: every drawing, conversion and value is the same, bit for bit. In the code, each calculation that mathematics names is now a call to a function of that name, with its source — the code reads as the theory it applies.

| Before                                                                 | After                                                                    |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `center.x + (dx * direction.x - dy * direction.y)`, and the same for y | `add(center, rotate(subtract(point, center), direction))`                |
| `Math.atan2(perpDot(d, h), dot(d, h))`, then `(r * 180) / Math.PI`     | `radiansToDegrees(signedAngle(direction, handle))`                       |
| `Math.atan2(scale * Math.sin(θ), Math.cos(θ))`, θ = `(2π(k + 1)) / n`  | the polar angle of the regular direction stretched by a Hadamard product |
| `(index * end) / SAMPLES` in three places                              | `regularPartition(end, SAMPLES)`                                         |

## In numbers

| Item                     | Count                                                                                                                                                                                 |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| New named functions      | 33 modules: distance, rotate, Hadamard product, scalar projection, degrees and radians, signed angle, modulo, discriminant, Bernstein weights, regular partition, box size, has area… |
| Duplicates merged        | the empty handles and corners, the shape kinds told apart four ways, a stretch written twice, a test helper                                                                           |
| Left inline, with reason | about 20, where naming would change the order of floating-point operations, or nothing has a name                                                                                     |
| Proof                    | a golden file of 400 results compared bit for bit after each story; it caught a reordered product on purpose                                                                          |

Sources and the full lists: [research note 0016](../research/notes/0016-f13-named-calculations.md).

## How to check it

Nothing to try in the playground: every drawing is the one of F10's validation. The demo page of F10 holds the guided test of both.

## Validation (2026-10-14)

Accepted by the Product Owner with F10, without remark. They asked that its work become a standing rule: every calculation named while writing, and a naming pass over the whole code at the end of each epic ([ADR-0049](../adr/0049-every-calculation-named.md)).
