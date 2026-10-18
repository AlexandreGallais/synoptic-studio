# Rule catalogue

Lint and format rules worth turning on in some kind of project, sorted into categories, so that the "method and tools" point of a review (ADR-0074, held at each return of the Product Owner since ADR-0086) or a new project's start can switch whole categories on from what is built and with which technology. **On paper only**: nothing on this page is enabled by it; this repository's own choice is [rules](./rules.md). Asked by the Product Owner on 2026-10-17 (EN-093).

Each row names a category, the rules it holds and why they serve the agent's own work: a bug found by a check in under a second rather than by a test, a review or a person; a form chosen once, so that code is written and read without a choice; a context kept whole. A rule that only guards what a kept rule or the language already forbids is listed at the end with a word.

Rule lists read on 2026-10-17: ESLint's core rules (`REF-ESLINT-RULES`), typescript-eslint 8.71.2 (`REF-TSESLINT-RULES`), eslint-plugin-unicorn (`REF-UNICORN-RULES`), eslint-plugin-jsdoc (`REF-JSDOC-RULES`), `@eslint/markdown` (`REF-ESLINT-MARKDOWN-RULES`), oxlint 1.87.0's own rules (`REF-OXLINT-RULE-LIST`), angular-eslint's TypeScript and template rules (`REF-ANGULAR-ESLINT-RULES`, `REF-ANGULAR-ESLINT-TEMPLATE-RULES`). A rule of ESLint, typescript-eslint, Unicorn or JSDoc runs natively in oxlint when it has it, in the project's linter otherwise (`tools/lint`); angular-eslint's rules have no native runner yet.

## Categories

```text
TypeScript (always: no plain JavaScript)
├── correctness            always
├── type safety            always
├── consistency            always
├── naming                 always
├── size guardrails        always
├── library                a published API
├── functional             pure, immutable, no class
├── business objects       classes, `let`, `enum` allowed
├── Node.js tools          scripts, hooks, CLIs
├── browser and DOM        a page in a browser
├── tests                  Vitest
└── Angular
    ├── TypeScript         components, services, signals
    ├── template           control flow, bindings
    ├── i18n               `$localize`, Transloco
    └── accessibility      an option
Markdown
Formatting
```

## TypeScript

### Correctness — always

| Rules | Why |
| --- | --- |
| ESLint: `array-callback-return`, `for-direction`, `no-async-promise-executor`, `no-compare-neg-zero`, `no-cond-assign`, `no-constant-binary-expression`, `no-constant-condition`, `no-control-regex`, `no-debugger`, `no-dupe-else-if`, `no-duplicate-case`, `no-empty-character-class`, `no-empty-pattern`, `no-ex-assign`, `no-fallthrough`, `no-invalid-regexp`, `no-irregular-whitespace`, `no-loss-of-precision`, `no-misleading-character-class`, `no-promise-executor-return`, `no-prototype-builtins`, `no-self-assign`, `no-self-compare`, `no-sparse-arrays`, `no-template-curly-in-string`, `no-unmodified-loop-condition`, `no-unreachable`, `no-unreachable-loop`, `no-unsafe-finally`, `no-unsafe-optional-chaining`, `no-unused-vars`, `no-use-before-define`, `no-useless-backreference`, `require-atomic-updates`, `use-isnan`, `preserve-caught-error`, `no-useless-catch` | each is code that compiles and is wrong; the check costs a second, the bug a test round or a review |
| oxlint: `oxc/bad-comparison-sequence`, `bad-match-all-arg`, `bad-replace-all-arg`, `bad-min-max-func`, `const-comparisons`, `double-comparisons`, `erasing-op`, `misrefactored-assign-op`, `missing-throw`, `number-arg-out-of-range`, `only-used-in-recursion`, `approx-constant` | the same; oxlint's own, no ESLint plugin has them |
| Unicorn: the `no-invalid-*` rules (`argument-count`, `boolean-attribute-value`, `character-comparison`, `dom-token`, `fetch-options`, `file-input-accept`, `integrity`, `intl-options`, `property-descriptor`, `remove-event-listener`, `response-options`, `style-set-property`, `temporal-arithmetic`, `url-protocol-comparison`, `well-known-symbol-methods`), `no-incorrect-query-selector`, `no-incorrect-template-string-interpolation`, `no-mismatched-map-key`, `no-impossible-length-comparison`, `no-loop-iterable-mutation`, `no-multiple-promise-resolver-calls`, `no-uncalled-method`, `no-unused-builtin-method-return`, `no-unused-iterator-helper`, `no-useless-recursion`, `no-xor-as-exponentiation`, `no-accidental-bitwise-operator`, `no-chained-comparison`, `no-redundant-comparison`, `no-double-comparison`, `no-duplicate-if-branches`, `no-duplicate-logical-operands`, `no-duplicate-set-values`, `no-thenable`, `no-unsafe-string-replacement`, `no-unsafe-property-key`, `require-array-join-separator`, `require-number-to-fixed-digits-argument`, `require-proxy-trap-boolean-return`, `prefer-math-constants` | API misuse a type check lets through |

### Type safety — always

| Rules | Why |
| --- | --- |
| typescript-eslint: `no-explicit-any`, `no-non-null-assertion`, `consistent-type-assertions`, `no-unsafe-type-assertion`, `no-unsafe-argument`, `no-unsafe-assignment`, `no-unsafe-call`, `no-unsafe-member-access`, `no-unsafe-return`, `no-unsafe-function-type`, `no-unsafe-unary-minus`, `no-wrapper-object-types`, `ban-ts-comment`, `ban-tslint-comment` | a type the agent cannot trust is a type it must check by reading every caller |
| typescript-eslint: `no-floating-promises`, `no-misused-promises`, `await-thenable`, `return-await`, `only-throw-error`, `prefer-promise-reject-errors`, `promise-function-async`, `require-await` | a promise not awaited fails later, elsewhere, without a trace |
| typescript-eslint: `strict-boolean-expressions`, `no-unnecessary-condition`, `switch-exhaustiveness-check`, `no-unnecessary-boolean-literal-compare`, `restrict-plus-operands`, `restrict-template-expressions`, `no-base-to-string`, `unbound-method`, `no-deprecated`, `no-array-delete`, `no-dynamic-delete`, `no-misused-spread`, `require-array-sort-compare`, `no-confusing-void-expression`, `no-meaningless-void-operator`, `no-invalid-void-type`, `strict-void-return`, `use-unknown-in-catch-callback-variable`, `no-implied-eval`, `no-redundant-type-constituents`, `no-duplicate-type-constituents`, `no-unnecessary-type-assertion`, `no-unnecessary-type-arguments`, `no-unnecessary-type-constraint`, `no-unnecessary-type-conversion`, `no-unnecessary-type-parameters`, `no-unnecessary-template-expression`, `no-useless-default-assignment` | a case the compiler accepts but the types say cannot happen, or code that does nothing |

### Consistency — always

One form for each construct, most of them fixed by `--fix`: the agent never chooses between two ways, and a diff holds only what changed.

| Rules | Why |
| --- | --- |
| ESLint: `curly`, `eqeqeq`, `yoda`, `object-shorthand`, `prefer-template`, `prefer-arrow-callback`, `arrow-body-style`, `func-style`, `func-names`, `func-name-matching`, `logical-assignment-operators`, `operator-assignment`, `prefer-exponentiation-operator`, `prefer-object-spread`, `prefer-object-has-own`, `prefer-numeric-literals`, `prefer-regex-literals`, `prefer-named-capture-group`, `require-unicode-regexp`, `prefer-rest-params`, `prefer-spread`, `prefer-destructuring`, `no-else-return`, `no-lonely-if`, `no-unneeded-ternary`, `no-useless-rename`, `no-useless-computed-key`, `no-useless-return`, `no-useless-concat`, `no-useless-escape`, `no-extra-boolean-cast`, `no-implicit-coercion`, `default-case-last`, `default-param-last`, `sort-imports` (members only) | the shortest correct form, always the same |
| typescript-eslint: `array-type`, `consistent-type-definitions`, `consistent-indexed-object-style`, `consistent-generic-constructors`, `method-signature-style`, `consistent-type-imports`, `consistent-type-exports`, `no-import-type-side-effects`, `no-inferrable-types`, `prefer-optional-chain`, `prefer-nullish-coalescing`, `prefer-as-const`, `prefer-function-type`, `prefer-find`, `prefer-for-of`, `prefer-includes`, `prefer-regexp-exec`, `prefer-reduce-type-parameter`, `prefer-string-starts-ends-with`, `dot-notation`, `adjacent-overload-signatures`, `unified-signatures`, `no-empty-object-type`, `triple-slash-reference`, `no-namespace` | one way to write a type and to import it |
| Unicorn: the `prefer-*` rules of arrays, strings, numbers, sets and iterators (`prefer-at`, `prefer-array-some`, `prefer-array-find`, `prefer-array-flat-map`, `prefer-string-slice`, `prefer-string-replace-all`, `prefer-set-has`, `prefer-iterator-helpers`, `prefer-math-min-max`, `prefer-number-properties`, …), `no-negated-condition`, `no-useless-else`, `prefer-early-return`, `prefer-ternary`, `prefer-else-if`, `prefer-switch`, `prefer-hoisting-branch-code`, `prefer-combined-guards`, `prefer-simple-condition-first`, `switch-case-braces`, `switch-case-break-position`, `numeric-separators-style`, `number-literal-case`, `escape-case`, `no-zero-fractions`, `text-encoding-identifier-case`, `catch-error-name`, `throw-new-error`, `new-for-builtins`, `prefer-optional-catch-binding`, `no-useless-undefined`, `no-useless-spread`, `no-useless-length-check`, `no-unreadable-array-destructuring`, `no-unreadable-iife`, `no-unreadable-new-expression`, `no-unreadable-for-of-expression`, `no-array-callback-reference`, `no-array-method-this-argument`, `no-await-expression-member` | the same, for the built-in objects |
| Import: `first`, `newline-after-import`, `no-duplicates`, `no-empty-named-blocks`, `no-namespace`, `no-unassigned-import`, `consistent-type-specifier-style`, `no-absolute-path`, `no-self-import`, `no-cycle` | one import layout; a cycle is a load order bug |
| JSDoc layout: `check-alignment`, `check-indentation`, `check-line-alignment`, `multiline-blocks`, `no-multi-asterisks`, `require-asterisk-prefix`, `require-hyphen-before-param-description`, `sort-tags`, `tag-lines`, `lines-before-block`, `no-bad-blocks`, `no-blank-blocks`, `no-blank-block-descriptions`, `convert-to-jsdoc-comments` | blocks fixed into one shape, diffable |
| comments: `no-inline-comments`, `no-warning-comments`, `@stylistic/spaced-comment`, `@stylistic/multiline-comment-style`, `@stylistic/padding-line-between-statements`, Unicorn `no-abusive-eslint-disable`, `comment-content` | a comment above its line is read before the code; an exception says why |

### Naming — always

| Rules | Why |
| --- | --- |
| typescript-eslint `naming-convention`; ESLint `id-length`, `id-denylist`, `no-underscore-dangle`; Unicorn `filename-case`, `name-replacements`, `consistent-compound-words`, `no-keyword-prefix`, `consistent-boolean-name`, `no-non-function-verb-prefix`, `consistent-tuple-labels` | names are what the agent searches for; a boolean named `isX` and a full word need no look at a type or a definition |

### Size guardrails — always, the limits by context

| Rules | Why |
| --- | --- |
| ESLint `complexity`, `max-depth`, `max-lines`, `max-lines-per-function`, `max-params`, `max-statements`, `max-nested-callbacks`, `no-nested-ternary`; Unicorn `max-nested-calls`, `try-complexity`; Import `max-dependencies` | guardrails that flag a function or a module holding two contexts; never a reason to cut one context into invented halves ([functions](./functions.md#one-business-context-per-function)): the limits are set per kind of function, looser in tests |

### Library — a published API

| Rules | Why |
| --- | --- |
| typescript-eslint `explicit-function-return-type`, `explicit-module-boundary-types` | the declarations and the API pages are read from the signatures, without inference (isolated declarations) |
| JSDoc presence and content: `require-jsdoc`, `require-description`, `require-param`, `require-param-description`, `require-param-name`, `require-returns`, `require-returns-description`, `require-returns-check`, `require-throws`, `require-throws-description`, `require-rejects`, `require-yields`, `require-yields-check`, `require-template`, `require-template-description`, `require-next-description`, `informative-docs`, `match-description`, `check-param-names`, `check-template-names`, `check-tag-names`, `check-syntax`, `empty-tags`, `escape-inline-tags`, `no-defaults`, `no-types`, `normalize-see-links`, `no-restricted-syntax` | the block is what the agent and the user read instead of the body; `informative-docs` refuses a description that only repeats the name |
| Import `no-default-export`, `no-anonymous-default-export`, `no-named-as-default`, `no-named-as-default-member`, `no-named-default`, `no-nodejs-modules`; ESLint `no-restricted-exports`, `no-console`, `no-restricted-globals`, `no-restricted-properties` | one name per export, found by a search; a portable core with no host |
| oxlint `oxc/no-barrel-file` | for a large application only, where barrels slow the bundler and the tests; a small library keeps them (ADR-0012, measured in [rules](./rules.md#barrels-adr-0012-measured)) |

### Functional — pure, immutable, no class

| Rules | Why |
| --- | --- |
| functional: `no-let`, `immutable-data`, `prefer-immutable-types`, `type-declaration-immutability`, `readonly-type`, `no-classes`, `no-this-expressions`, `no-mixed-types`, `functional-parameters`, `no-expression-statements`, `no-loop-statements`, `no-throw-statements`, `no-try-statements`, `no-conditional-statements`, `no-return-void`, `no-promise-reject` | a value that cannot change needs no tracing; a function with no effect is tested by its examples alone |
| ESLint `no-param-reassign`, `no-plusplus`, `no-restricted-syntax` (`enum`, `for…in`, accessors); typescript-eslint `no-restricted-types` (`Map` → `ReadonlyMap`); Unicorn `no-null`, `no-array-reverse`, `no-array-sort`, `no-array-for-each`, `no-immediate-mutation`; oxlint `oxc/no-accumulating-spread` | the same, for the language's mutating forms; a fold that copies its accumulator is quadratic |
| off here: ESLint `prefer-const` (no `let` to fix), the class and `enum` rules of the next section, Unicorn `no-array-reduce` (the fold is the loop of this style), oxlint `oxc/no-map-spread` (it asks for a change in place) | — |

### Business objects — classes, `let`, `enum` allowed

A domain modelled with objects that hold state and behaviour, or a framework that asks for classes (Angular).

| Rules | Why |
| --- | --- |
| `let`: ESLint `prefer-const`, `no-loop-func`, `no-shadow`, `init-declarations`, `no-unassigned-vars`, `no-useless-assignment` | `let` where a value changes, `const` everywhere else, fixed: the reassigned names stand out |
| classes: typescript-eslint `explicit-member-accessibility`, `member-ordering`, `prefer-readonly`, `parameter-properties`, `class-literal-property-style`, `class-methods-use-this`, `no-extraneous-class`, `no-useless-constructor`, `no-misused-new`, `no-this-alias`, `prefer-return-this-type`, `no-unsafe-declaration-merging`, `no-unused-private-class-members`; ESLint `no-invalid-this`, `accessor-pairs`, `grouped-accessor-pairs`, `max-classes-per-file`, `no-constructor-return`, `no-empty-static-block`; Unicorn `prefer-class-fields`, `prefer-private-class-fields`, `consistent-class-member-order`, `no-static-only-class`, `no-this-assignment`, `no-this-outside-of-class`, `no-accessor-recursion`, `no-incomplete-accessor-override`, `no-undeclared-class-members`, `no-useless-override`, `custom-error-definition`, `class-reference-in-static-methods` | members always in one order and visibility, so that a class is read top-down like a module; the bugs of `this` and accessors |
| `enum`: typescript-eslint `no-duplicate-enum-values`, `prefer-enum-initializers`, `prefer-literal-enum-member`, `no-mixed-enums`, `no-unsafe-enum-comparison`, `no-unsafe-enum-assignment`; oxlint `oxc/no-const-enum` | an `enum` whose values are written and never compared with a raw number; a `const enum` breaks isolated modules |

### Node.js tools — scripts, hooks, command lines

| Rules | Why |
| --- | --- |
| Unicorn `prefer-node-protocol`, `prefer-module`, `prefer-top-level-await`, `prefer-import-meta-properties`, `prefer-json-import`, `consistent-json-file-read`, `no-unsafe-json-serialization`, `require-text-decoder-streaming`, `no-unsafe-buffer-conversion`, `no-new-buffer`, `prefer-structured-clone`, `explicit-timer-delay`; oxlint `node/no-path-concat`, `node/handle-callback-err`, `node/no-new-require`, `promise/no-return-in-finally`, `promise/no-multiple-resolved`, `promise/valid-params`, `promise/no-callback-in-promise` | the platform's current forms; a path built by concatenation breaks on another OS |
| off here: Import `no-nodejs-modules`, ESLint `no-console` (a tool talks on its streams), Unicorn `no-process-exit` (a command line sets its exit code), the size guardrails of the library (a tool is a procedure) | — |

### Browser and DOM

| Rules | Why |
| --- | --- |
| ESLint `no-alert`, `no-eval`, `no-new-func`, `no-script-url`, `no-implied-eval`; Unicorn's DOM and web rules (`prefer-add-event-listener`, `prefer-dom-node-append`, `prefer-dom-node-remove`, `prefer-dom-node-text-content`, `prefer-query-selector`, `prefer-modern-dom-apis`, `prefer-keyboard-event-key`, `prefer-event-target`, `prefer-classlist-toggle`, `prefer-toggle-attribute`, `no-unsafe-dom-html`, `no-late-current-target-access`, `no-late-event-control`, `no-prevent-default-in-passive-listener`, `require-passive-events`, `prefer-add-event-listener-options`, `no-document-cookie`, `require-post-message-target-origin`, `prefer-blob-reading-methods`, `no-blob-to-file`, `prefer-url-can-parse`, `prefer-url-search-parameters`, `no-url-in-search-params`, `relative-url-style`, `require-css-escape`, `prefer-https`) | the safe and current form of each DOM call; `innerHTML` and `postMessage` without an origin are security bugs |

### Tests — Vitest

| Rules | Why |
| --- | --- |
| oxlint's `vitest/`: `no-focused-tests`, `no-disabled-tests`, `no-identical-title`, `valid-title`, `valid-describe-callback`, `valid-expect-in-promise`, `require-awaited-expect-poll`, `no-conditional-expect`, `no-conditional-tests`, `no-duplicate-hooks`, `hoisted-apis-on-top`, `expect-expect` (with the names of the project's assertion helpers) | a test that does not run, runs twice under one name, or asserts nothing, passes in silence; a `.only` left in a commit hides the rest of the suite |
| oxlint's `vitest/`: `consistent-test-it`, `prefer-each`, `prefer-to-be`, `prefer-to-have-length`, `prefer-to-contain`, `prefer-called-times`, `prefer-strict-equal`, `prefer-hooks-on-top`, `prefer-hooks-in-order` | one form for each assertion |
| looser guardrails in test files: `max-lines`, `max-lines-per-function`, `max-statements`, `no-magic-numbers` off, `max-nested-callbacks` raised; JSDoc `require-param` and `require-returns` off for a helper read by its test alone | a test is a list of cases; its numbers are the hand-computed values |

### Angular — TypeScript

| Rules | Why |
| --- | --- |
| `computed-must-return`, `contextual-lifecycle`, `no-async-lifecycle-method`, `require-lifecycle-on-prototype`, `reactive-context-must-read-signal`, `no-uncalled-signals`, `no-implicit-take-until-destroyed`, `no-lifecycle-call`, `no-pipe-impure`, `no-duplicates-in-metadata-arrays`, `no-experimental`, `no-developer-preview` | the framework's bugs: a signal read as a value, a lifecycle hook that never runs, a subscription never released |
| `prefer-standalone`, `prefer-inject`, `inject-at-top`, `prefer-signals`, `prefer-signal-model`, `prefer-output-emitter-ref`, `prefer-output-readonly`, `prefer-on-push-component-change-detection`, `prefer-host-metadata-property`, `use-injectable-provided-in`, `use-lifecycle-interface`, `use-pipe-transform-interface`, `sort-lifecycle-methods`, `sort-keys-in-type-decorator`, `consistent-component-styles`, `contextual-decorator`, `relative-url-prefix` | the current form of each Angular construct, always the same; a component reads like the next one |
| `component-selector`, `directive-selector`, `pipe-prefix`, `use-component-selector`, `no-input-prefix`, `no-input-rename`, `no-output-rename`, `no-output-native`, `no-output-on-prefix`, `no-inputs-metadata-property`, `no-outputs-metadata-property`, `no-queries-metadata-property`, `no-attribute-decorator`, `no-forward-ref`, `use-component-view-encapsulation`, `component-max-inline-declarations`, `no-empty-lifecycle-method` | names found by a search across templates and classes; one place per input and output |
| left off: `component-class-suffix`, `directive-class-suffix` (no longer recommended since angular-eslint 20), `prefer-service-decorator` until the project's Angular has `@Service()` | — |

### Angular — template

| Rules | Why |
| --- | --- |
| `banana-in-box`, `no-duplicate-attributes`, `no-nested-tags`, `no-outerhtml`, `no-negated-async`, `no-any`, `no-non-null-assertion`, `eqeqeq`, `no-empty-control-flow`, `require-switch-default`, `button-has-type` | template bugs the compiler lets through |
| `prefer-control-flow`, `prefer-at-else`, `prefer-at-empty`, `prefer-contextual-for-variables`, `prefer-self-closing-tags`, `prefer-static-string-properties`, `prefer-class-binding`, `prefer-style-binding`, `prefer-template-literal`, `prefer-ngsrc`, `prefer-built-in-pipes`, `no-interpolation-in-attributes`, `attributes-order`, `no-inline-styles`, `no-call-expression`, `use-track-by-function` | one form for each template construct; a call in a template runs at every change detection |
| `conditional-complexity`, `cyclomatic-complexity` | the size guardrails, for templates |

### Angular — i18n

| Rules | Why |
| --- | --- |
| template `i18n` (attributes, custom ids, text left untranslated); TypeScript `require-localize-metadata`, `runtime-localize` | for Angular's own i18n (`$localize`): a text shipped untranslated or a message without its meaning and id is found before a translator meets it |
| a project on Transloco | the lists read hold no rule for Transloco's keys or pipes: the template `i18n` rule reads Angular's `i18n` attributes, which Transloco does not use; Transloco's own tooling (missing and unused keys) is to read first, then decided |

### Angular — accessibility (an option)

| Rules | Why |
| --- | --- |
| template `alt-text`, `elements-content`, `label-has-associated-control`, `click-events-have-key-events`, `mouse-events-have-key-events`, `interactive-supports-focus`, `no-autofocus`, `no-distracting-elements`, `no-positive-tabindex`, `role-has-required-aria`, `valid-aria`, `table-scope` | each is a failure a screen reader or a keyboard meets; checked in the template rather than found by an audit |

## Markdown

| Rules | Why |
| --- | --- |
| `@eslint/markdown`: `heading-increment`, `no-multiple-h1`, `no-duplicate-headings`, `no-missing-atx-heading-space`, `no-missing-link-fragments`, `no-missing-label-refs`, `no-invalid-label-refs`, `no-duplicate-definitions`, `no-unused-definitions`, `no-empty-definitions`, `no-empty-links`, `no-empty-images`, `no-reversed-media-syntax`, `no-space-in-emphasis`, `no-reference-like-urls`, `require-alt-text`, `table-column-count`, `fenced-code-language`, `fenced-code-meta`, `no-bare-urls`, `no-html` | a page whose links, anchors and tables are checked is a page the agent can follow without opening it in a browser; `no-html` keeps pages portable |
| Unicorn `require-frontmatter-fields` | for a site whose pages need fields (a title, a status): the backlog's front matter is such a case |

## Formatting

| Setting | Why |
| --- | --- |
| one formatter for every file kind (oxfmt), run by the fix and at the commit | the layout is never a choice nor a review comment |
| a print width measured on the project's own edits (100 here) | the smallest change for a small edit and the fewest conflicts ([rules](./rules.md#the-formatters-settings)) |
| trailing commas, one property per line once wrapped, sorted imports | adding an item changes one line |
| Markdown tables without padding, paragraphs kept on their lines | a wider cell changes its own row (ADR-0068) |

## Skipped, with a word

| Rules | Why skipped |
| --- | --- |
| ESLint's deprecated layout rules (`indent`, `quotes`, `semi`, `comma-dangle`, `max-len`, `padded-blocks`, …) | the formatter writes the layout |
| `no-var`, `vars-on-top`, `block-scoped-var`, `no-redeclare`, `no-undef-init`, `one-var`, `sort-vars` | `var` is banned; `const` and `let` are block-scoped |
| `constructor-super`, `getter-return`, `no-class-assign`, `no-const-assign`, `no-delete-var`, `no-dupe-args`, `no-dupe-class-members`, `no-dupe-keys`, `no-func-assign`, `no-import-assign`, `no-new-native-nonconstructor`, `no-obj-calls`, `no-setter-return`, `no-this-before-super`, `no-unsafe-negation`, `no-with`, `no-undef`, `no-octal`, `no-octal-escape`, `no-nonoctal-decimal-escape`, `strict` | TypeScript refuses the code (typescript-eslint's `eslint-recommended` turns them off for that reason) |
| JSDoc `check-types`, `valid-types`, `no-undefined-types`, `require-param-type`, `require-returns-type`, `require-property-type`, `require-throws-type`, `require-yields-type`, `require-next-type`, `type-formatting`, `reject-any-type`, `reject-function-type`, the `ts-*` rules | the types are in the signature, not in the block |
| `no-ternary`, `no-negated-condition` beside Unicorn's, `no-undefined`, `sort-keys`, `capitalized-comments`, `id-match`, oxlint `oxc/no-async-await`, `no-optional-chaining`, `no-rest-spread-properties` | a taste or a target older than the language, not a guard |
| typescript-eslint `no-type-alias`, `typedef`, `sort-type-constituents`, `prefer-ts-expect-error`, `no-var-requires`, `no-require-imports` | the opposite of the consistency rules kept, or CommonJS |
| Unicorn `no-for-loop`, `prefer-continue`, `prefer-temporal`, `prefer-dispose`, `expiring-todo-comments` | another rule of the profile decides it (loops banned or allowed, `TODO` banned), or the runtime lacks the API |

## Profiles

Ready sets of categories, picked at a project's start from what the person says they build and with which technology. Each turns on the "always" categories of TypeScript (correctness, type safety, consistency, naming, size guardrails), Markdown and formatting, then:

| Profile | Turns on | Turns off | Why |
| --- | --- | --- | --- |
| functional library (this repository) | library, functional, tests | business objects, `prefer-const`, Node.js tools' freedoms in the core | pure functions over plain data, each read and tested alone; the published API documented from its blocks |
| business library | library, business objects, tests | functional's `no-let`, `no-classes`, `no-this-expressions`, `no-loop-statements`, `no-throw-statements`, `no-try-statements`, `no-conditional-statements`, `no-expression-statements`; Unicorn `no-null` when a persistence layer speaks `null` | domain objects keep state and behaviour together; the class rules give them one shape, `prefer-const` keeps `let` rare; immutability stays where the domain is a value (`readonly` types, `no-param-reassign`) |
| Angular application | business objects, browser and DOM, tests, Angular TypeScript, template; i18n when the application is translated (Angular's own i18n, or Transloco with its tooling); accessibility when the application has users with assistive technologies, or by default for a public one | library (no published API: JSDoc presence on public members only, no `explicit-module-boundary-types`), functional; `oxc/no-barrel-file` on for a large application | Angular asks for classes, decorators and injection; its signals and templates have their own bugs; size guardrails apply to templates too |
| tooling and scripts | Node.js tools, tests | library, browser and DOM, functional's statement rules, `no-console`, `no-process-exit`, `no-nodejs-modules` | a tool is a procedure that reads files and prints: its effects are its job; correctness and type safety stay whole, since a tool's bug blocks every commit |
