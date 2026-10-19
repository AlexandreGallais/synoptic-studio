# ADR-0093 — The editor's TypeScript, the installed TypeScript 7

**Status**: Accepted (the agent, 2026-10-19, AUD-035, from the audit's finding F9; to be told to the Product Owner). Supersedes, in ADR-0075's Consequences, the sentence "`.vscode/settings.json` no longer points VS Code at `node_modules/typescript/lib`, where TypeScript 7's package holds no `tsserver.js` to load". The rest of that consequence, the editors on Oxc's extension, and ADR-0075's decisions stand.

## Context

- ADR-0075 made TypeScript 7 the only TypeScript and removed VS Code's pointer to `node_modules/typescript/lib`: TypeScript 7's package holds no `tsserver.js`, so VS Code's built-in TypeScript could not load it, and the editor fell back on the TypeScript it ships.
- TypeScript 7 has its own VS Code extension, `TypeScriptTeam.native-preview`, "the native implementation of the TypeScript language service": `"js/ts.experimental.useTsgo": true` enables it, and `"js/ts.tsdk.path": "./node_modules/typescript"` makes it use a local TypeScript package (`REF-TS7-VSCODE`).
- EN-121 set both settings in `.vscode/settings.json` and recommended the extension in `.vscode/extensions.json`, so that the editor checks with the TypeScript 7.0.2 the lint runs; `[F33.AC1]` of `tests/typescript-7.test.ts` holds them. ADR-0075's sentence then said the opposite of the repository (AUD-035, F9).

## Decision

1. **The editor's TypeScript is the installed TypeScript 7**, through its own extension: `.vscode/extensions.json` recommends `TypeScriptTeam.native-preview` beside `oxc.oxc-vscode` and `editorconfig.editorconfig`, and `.vscode/settings.json` sets `"js/ts.experimental.useTsgo": true` and `"js/ts.tsdk.path": "./node_modules/typescript"` (`REF-TS7-VSCODE`).
2. **The path names the package folder**, never its `lib/`: the extension reads TypeScript 7's package, which has no `tsserver.js` for VS Code's built-in TypeScript.

## Reasons

- The editor reports the errors the lint reports, from the same compiler at the same version, rather than from the TypeScript VS Code ships.
- The two settings are the extension's own, as its page gives them; the test fails if either goes or if the folder holds another major version.

| Option | For | Against |
| --- | --- | --- |
| TypeScript 7's extension on the installed package (chosen) | the lint's compiler in the editor; held by a test | one more extension to install |
| VS Code's built-in TypeScript (ADR-0075 as written) | nothing to install | another compiler and version than the lint's, its errors not the same |
| `js/ts.tsdk.path` to `node_modules/typescript/lib` | VS Code's usual setting | TypeScript 7's package has no `tsserver.js` there to load |

## Consequences

- [formatting](../conventions/formatting.md) lists the three recommended extensions and the two settings; ADR-0075's status in the [index](./) names this decision.
- Another extension for TypeScript 7, or a renamed setting, is a change of `.vscode/` and of `tests/typescript-7.test.ts`, with this ADR superseded.

## References

ADR-0075, `REF-TS7-VSCODE`, EN-121, AUD-035
