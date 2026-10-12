import type { TSESLint } from "@typescript-eslint/utils";

/** Runtime platform APIs: Node.js, binary data, time, text encoding. */
export const UNICORN_PLATFORM: TSESLint.FlatConfig.Config = {
  name: "unicorn/platform",
  rules: {
    "unicorn/consistent-date-clone": "error",
    "unicorn/consistent-json-file-read": "error",
    "unicorn/explicit-timer-delay": "error",
    "unicorn/no-invalid-intl-options": "error",
    "unicorn/no-invalid-temporal-arithmetic": "error",
    "unicorn/no-new-buffer": "error",
    "unicorn/no-process-exit": "error",
    "unicorn/no-unsafe-buffer-conversion": "error",
    "unicorn/no-unsafe-json-serialization": "error",
    "unicorn/no-unsafe-sqlite-interpolation": "error",
    "unicorn/no-using-resource-escape": "error",
    "unicorn/prefer-date-now": "error",
    "unicorn/prefer-dispose": "error",
    "unicorn/prefer-node-protocol": "error",
    "unicorn/prefer-structured-clone": "error",
    "unicorn/prefer-temporal": "error",
    "unicorn/prefer-temporal-conversion": "error",
    "unicorn/prefer-uint8array-base64": "error",
    "unicorn/prefer-uint8array-hex": "error",
    "unicorn/require-text-decoder-streaming": "error",
    "unicorn/text-encoding-identifier-case": "error",
  },
};
