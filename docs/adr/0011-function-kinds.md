# ADR-0011 — Function kinds: `@kind` tag and per-kind limits

**Status**: Accepted

## Context

The Product Owner asked for a marker on every function, like an Angular decorator, stating its kind (mathematical, business…), and for ESLint to apply rules per kind: maximum number of lines, whether the name starts with a verb. Longer business procedures must stay possible when needed.

## Options considered

| Option | Verdict |
| --- | --- |
| TypeScript decorator `@math` | **Impossible**: a decorator only applies to classes and their members, never to a free function (`REF-TS-DECORATORS`, `REF-TC39-DECORATORS`). And classes are excluded from the project. |
| Kind inferred from the folder | Insufficient: `geometry/` holds `math` and `geometry`, `render/` holds `format` and `procedure`. |
| Suffix in the name (`dotMath`) | Pollutes names, contradicts "standard mathematical name". |
| **JSDoc tag `@kind`** | Read by ESLint, visible on hover in the IDE, documents at the same time. **Chosen.** |
| `@category` tag (TypeDoc) | Technically equivalent; discarded to keep the established vocabulary (`@kind`). Note: JSDoc 3 also defines a `@kind` (class, function…); the project redefines it, hence `jsdoc/check-values` is off. The tag is declared in `tsdoc.json` for TypeDoc. |

## Decision

Every top-level function carries exactly one `@kind` among `math`, `geometry`, `domain`, `format`, `procedure` (`local/require-kind`), allowed in its layer (`local/kind-in-layer`), and bounded by the limits of its kind (`local/kind-limits`):

| `@kind` | Lines | Statements | Complexity | Depth |
| --- | --- | --- | --- | --- |
| `math` | 10 | 4 | 3 | 1 |
| `geometry` | 20 | 8 | 5 | 2 |
| `domain` | 20 | 8 | 5 | 2 |
| `format` | 20 | 8 | 4 | 1 |
| `procedure` | **40** | **15** | **3** | **1** |

Naming per kind (`local/kind-naming`): a `procedure` starts with a verb from a closed list; `math`/`geometry`/`domain` reject vague verbs (`get`, `compute`…); `format` reads as a conversion (`xToY`, `toX`, `parseX`); every boolean function starts with a predicate word (`is`, `has`, `can`…), and only those do.

### Why a procedure may be longer, but not more complex

- Length is not the problem; the gap between intention and implementation is (`REF-FOWLER-FUNCTION-LENGTH`).
- A long business procedure must be a **Composed Method**: a sequence of calls to named steps, all at the same level of abstraction (`REF-BECK-SBPP`). Such a sequence is long but **linear**.
- Cyclomatic complexity counts independent paths (`REF-NIST-500-235`): branches make a function hard to test, not lines. Hence 40 lines allowed, but complexity 3 and depth 1.
- McCabe's historical threshold is 10, tolerated up to 15 (`REF-NIST-500-235`). The project is deliberately much stricter, because its functions are named concepts.

There is **no escape hatch** such as `@longform`: an exception allowed by annotation tends to spread. If a procedure exceeds its limits, extract steps; if that fails after two attempts, stop and propose a split (CLAUDE.md guardrail).

## Consequences

- The global core ESLint limits (`complexity`, `max-lines-per-function`…) are set to the most permissive kind; the fine limits are those of `local/kind-limits`.
- The complexity of `local/kind-limits` includes nested callbacks (stricter than the core rule).
- The values live in `eslint/settings/kinds.ts`; changing them requires updating this ADR (or superseding it).

## References

`REF-TS-DECORATORS`, `REF-TC39-DECORATORS`, `REF-FOWLER-FUNCTION-LENGTH`, `REF-BECK-SBPP`, `REF-NIST-500-235`
