import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** The playground's folder, an npm project of its own (ADR-0104). */
const PLAYGROUND = join(import.meta.dirname, "..", "playground");

/**
 * A JSON file of the playground, parsed.
 *
 * @param name - its name in the playground's folder
 * @returns its value
 */
function playgroundJson(name: string): unknown {
  return JSON.parse(readFileSync(join(PLAYGROUND, name), "utf8"));
}

/**
 * A value at a path of keys inside a parsed JSON value.
 *
 * @param value - a parsed JSON value
 * @param keys - the keys to follow, outermost first
 * @returns the value found, or `undefined`
 */
function at(value: unknown, keys: readonly string[]): unknown {
  return keys.reduce<unknown>(
    (found, key) =>
      typeof found === "object" && found !== null ? Reflect.get(found, key) : undefined,
    value,
  );
}

describe("the playground's project files", () => {
  it("[F50.AC2] sets the schematics' defaults: suffixed names, three files, SCSS", () => {
    const schematics = at(playgroundJson("angular.json"), ["projects", "playground", "schematics"]);

    expect(at(schematics, ["@schematics/angular:component"])).toMatchObject({
      inlineStyle: false,
      inlineTemplate: false,
      style: "scss",
      type: "component",
    });

    expect(at(schematics, ["@schematics/angular:directive", "type"])).toBe("directive");
    expect(at(schematics, ["@schematics/angular:service", "type"])).toBe("service");

    for (const kind of ["guard", "pipe", "interceptor", "resolver"]) {
      expect(at(schematics, [`@schematics/angular:${kind}`, "typeSeparator"]), kind).toBe(".");
    }
  });

  it("[F50.AC2] checks templates strictly, every extended diagnostic an error", () => {
    const angular = at(playgroundJson("tsconfig.json"), ["angularCompilerOptions"]);

    expect(at(angular, ["strictTemplates"])).toBe(true);
    expect(at(angular, ["extendedDiagnostics", "defaultCategory"])).toBe("error");
  });

  it("[F52.AC2] makes each of Angular's 18 extended diagnostics an error", () => {
    // The 18 checks of Angular's extended diagnostics (REF-ANGULAR-EXTENDED-DIAGNOSTICS): the 17
    // of its page and `controlFlowPreventingContentProjection`, which `@angular/compiler-cli`
    // 22.2.3 adds to `ExtendedTemplateDiagnosticName`; each takes its category from `checks`, else
    // from `defaultCategory`, and emits only under `strictTemplates`; `tsconfig.app.json` and
    // `tsconfig.spec.json` extend this file.
    const names = [
      "invalidBananaInBox",
      "nullishCoalescingNotNullable",
      "missingControlFlowDirective",
      "textAttributeNotBinding",
      "missingNgForOfLet",
      "suffixNotSupported",
      "optionalChainNotNullable",
      "skipHydrationNotStatic",
      "interpolatedSignalNotInvoked",
      "uninvokedFunctionInEventBinding",
      "unusedLetDeclaration",
      "unusedStandaloneImports",
      "unparenthesizedNullishCoalescing",
      "uninvokedTrackFunction",
      "missingStructuralDirective",
      "uninvokedFunctionInTextInterpolation",
      "deferTriggerMisconfiguration",
      "controlFlowPreventingContentProjection",
    ];
    const angular = at(playgroundJson("tsconfig.json"), ["angularCompilerOptions"]);
    const fallback = at(angular, ["extendedDiagnostics", "defaultCategory"]);

    expect(names).toHaveLength(18);
    expect(at(angular, ["strictTemplates"])).toBe(true);

    expect(
      names.filter(
        (name) => (at(angular, ["extendedDiagnostics", "checks", name]) ?? fallback) !== "error",
      ),
    ).toEqual([]);

    for (const file of ["tsconfig.app.json", "tsconfig.spec.json"]) {
      expect(at(playgroundJson(file), ["extends"]), file).toBe("./tsconfig.json");
      expect(at(playgroundJson(file), ["angularCompilerOptions"]), file).toBeUndefined();
    }
  });

  it("[F50.AC2] imports the library by its name through the tsconfig's paths, no vite.config", () => {
    expect(at(playgroundJson("tsconfig.json"), ["compilerOptions", "paths", "editor"])).toEqual([
      "../src/index.ts",
    ]);

    expect(readdirSync(PLAYGROUND).filter((name) => name.startsWith("vite.config"))).toEqual([]);
  });
});
