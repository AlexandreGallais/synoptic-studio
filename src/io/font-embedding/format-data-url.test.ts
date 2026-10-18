import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { formatDataUrl } from "./format-data-url";

describe("formatDataUrl (REF-RFC-2397, DERIV-font-embedding step 6)", () => {
  it("[F15.AC2] writes data:, the media type, ;base64, and the bytes in base 64", () => {
    // "foo" in base 64 is Zm9v (REF-RFC-4648, §10).
    expect(formatDataUrl("font/ttf", Uint8Array.from([102, 111, 111]))).toBe(
      "data:font/ttf;base64,Zm9v",
    );

    expect(formatDataUrl("font/otf", new Uint8Array(0))).toBe("data:font/otf;base64,");
  });

  test.prop({ bytes: fc.uint8Array({ maxLength: 64 }), isCff: fc.boolean() })(
    "[F15.AC2] writes a font of n bytes in 21 + 4 · ⌈n / 3⌉ characters",
    ({ bytes, isCff }) => {
      expect(formatDataUrl(isCff ? "font/otf" : "font/ttf", bytes)).toHaveLength(
        21 + 4 * Math.ceil(bytes.length / 3),
      );
    },
  );
});
