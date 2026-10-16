# Register of exceptions

Every place where the code departs from a lint rule, with its reason (ADR-0066). A rule is turned off for one place only, by `// eslint-disable-next-line rule -- reason` beside the code ([lint](./lint.md#one-off-exceptions)); this page lists each of them, so that the exceptions are seen together.

## Keeping it in step

- A new exception gets its row here in the same change; a removed one loses its row.
- `tests/exceptions.test.ts` reads every tracked `.ts` file: each `eslint-disable` directive needs a row of its file and its rules, and each row needs its directive and its function in the file. A failure names the file and the rules.
- One row per directive: a directive that turns off two rules at once is one row, its rules in one cell.

## Library

| File                                                         | Function                                                                                    | Rule                                                                                           | Reason                                                  |
| ------------------------------------------------------------ | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| `src/model/handles/are-valid-handles.test.ts`                | `refuses an unknown mode, a vector not finite, a switch not boolean`                        | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | data read from outside: an unknown mode                 |
| `src/model/handles/are-valid-handles.test.ts`                | `refuses an unknown mode, a vector not finite, a switch not boolean`                        | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | data read from outside: a switch that is not a boolean  |
| `src/model/layout-box/is-anchor.test.ts`                     | `[F07.AC1] refuses any other value on either axis`                                          | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | data read from outside may hold any string              |
| `src/model/regular-polygon/is-valid-regular-polygon.test.ts` | `[F06.AC1] accepts a stretched polygon and refuses a stretched value that is not a boolean` | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | data read from outside may hold any value               |
| `src/model/regular-polygon/is-valid-regular-polygon.test.ts` | `[F07.AC1] refuses an alignment other than min, mid and max`                                | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | data read from outside may hold any string              |
| `src/model/text/is-valid-text-decorations.test.ts`           | `[F16.AC1] refuses a %s that is not true or false`                                          | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | a value read from outside the type, for an underline    |
| `src/model/text/is-valid-text-decorations.test.ts`           | `[F16.AC1] refuses a %s that is not true or false`                                          | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | a value read from outside the type, for a strikethrough |

## Playground

| File                                                      | Function              | Rule                                                                                           | Reason                                                                                |
| --------------------------------------------------------- | --------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `playground/default-font/default-font.test.ts`            | `fakeCanvas`          | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | happy-dom has no canvas: the fake holds only what the page reads                      |
| `playground/fonts/import-font.test.ts`                    | `readBlock`           | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the block is written by `showText` from a `TextBlock`                                 |
| `playground/guided-steps/epic-15/f05-steps.test.ts`       | `readBlock`           | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the block is written by `showText` from a `TextBlock`                                 |
| `playground/guided-steps/epic-15/f14-steps.test.ts`       | `readBlock`           | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the block is written by `showText` from a `TextBlock`                                 |
| `playground/installed-fonts/listen-to-installed-fonts.ts` | `readLocalFontAccess` | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the DOM types do not hold the draft Local Font Access API, detected as Chrome advises |
| `playground/position/write-resized-position.ts`           | `readDrawnBox`        | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | written by `writeDrawnBox` only, from a valid shape                                   |
| `playground/redraw/schedule-redraw.test.ts`               | `readBlockHeight`     | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the block is written by `showText` from a `TextBlock`                                 |
| `playground/shape/read-stored-free-shape.ts`              | `readKeptText`        | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | written by `writeStoredFreeShape` only, from a `FreeShape`, and checked just after    |
| `playground/text-box/fit-to-box.test.ts`                  | `readModel`           | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | written by `JSON.stringify` of a `Text`                                               |
| `playground/text-placement/read-drawn-text.ts`            | `readDrawnText`       | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | written by `writeTextPosition` only, from a valid text                                |
| `playground/text-placement/turned-text-corner.test.ts`    | `readStored`          | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the page writes the model as JSON                                                     |
| `playground/text-placement/turned-text-in-place.test.ts`  | `readDrawn`           | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the page writes the model as JSON                                                     |
| `playground/text-placement/turned-text-in-place.test.ts`  | `readDrawn`           | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the page writes the block as JSON                                                     |
| `playground/text/read-text-box-options.ts`                | `readTextBoxOptions`  | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the page offers only the values of this type                                          |
| `playground/text/read-text-model.ts`                      | `readTextModel`       | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the page offers only the values of this type: the font style                          |
| `playground/text/read-text-model.ts`                      | `readTextModel`       | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the page offers only the values of this type: the horizontal alignment                |
| `playground/text/read-text-model.ts`                      | `readTextModel`       | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | the page offers only the values of this type: the text direction                      |
| `playground/text/text-box-fields.test.ts`                 | `readModel`           | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | written by `JSON.stringify` of a `Text`                                               |
| `playground/text/write-tool-placement.ts`                 | `readKeptPlacement`   | `@typescript-eslint/consistent-type-assertions`, `@typescript-eslint/no-unsafe-type-assertion` | written by `saveToolPlacement` or `TOOL_PLACEMENT.start` only                         |

## Tooling and tests

| File                             | Function         | Rule                           | Reason                                                    |
| -------------------------------- | ---------------- | ------------------------------ | --------------------------------------------------------- |
| `eslint/scopes/language.ts`      | `jsdoc`          | `import-x/no-named-as-default` | the package's default export is the plugin                |
| `eslint/settings/identifiers.ts` | `ANY_FORMAT`     | `unicorn/no-null`              | the option schema of `naming-convention` requires `null`  |
| `tests/barrels.test.ts`          | `runtimeExports` | `no-restricted-syntax`         | loading each re-exported module is the point of this test |
