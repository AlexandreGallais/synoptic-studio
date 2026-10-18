import { afterEach, describe, expect, it, vi } from "vitest";

import config from "../vite.config";

/**
 * Vitest's options, as `vitest run` resolves them under the environment of the call.
 *
 * @returns the `test` options of the configuration
 */
function testOptions(): ReturnType<typeof config>["test"] {
  return config({ command: "serve", mode: "test" }).test;
}

describe("Vitest's reporters in the CI (EN-124)", () => {
  afterEach(() => {
    vi.unstubAllEnvs();
  });

  it("[F24.AC2] reports file by file, annotates a failure at its line and writes the JUnit report", () => {
    // `VITEST_JUNIT` is set by the CI's test job only: the default reporter prints each file as it
    // ends, the GitHub Actions one annotates each failure (`REF-VITEST-REPORTERS`).
    vi.stubEnv("VITEST_JUNIT", "test-report.xml");

    expect(testOptions()?.reporters).toStrictEqual(["default", "github-actions", "junit"]);
    expect(testOptions()?.outputFile).toStrictEqual({ junit: "test-report.xml" });
  });

  it("keeps Vitest's own reporters on the machine, where no JUnit report is asked", () => {
    vi.stubEnv("VITEST_JUNIT", undefined);

    expect(testOptions()?.reporters).toBeUndefined();
    expect(testOptions()?.outputFile).toBeUndefined();
  });
});
