import type { TSESLint } from "@typescript-eslint/utils";

/** Web platform APIs: fetch, URLs, HTML attributes, CSS, Markdown. */
export const UNICORN_WEB: TSESLint.FlatConfig.Config = {
  name: "unicorn/web",
  rules: {
    "unicorn/no-blob-to-file": "error",
    "unicorn/no-conflicting-constraints": "error",
    "unicorn/no-document-cookie": "error",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/no-empty-link-text": "off",
    "unicorn/no-ineffective-csp-directives": "error",
    "unicorn/no-invalid-boolean-attribute-value": "error",
    "unicorn/no-invalid-fetch-options": "error",
    "unicorn/no-invalid-file-input-accept": "error",
    "unicorn/no-invalid-integrity": "error",
    "unicorn/no-invalid-response-options": "error",
    "unicorn/no-invalid-url-protocol-comparison": "error",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/no-javascript-url": "off",
    "unicorn/no-missing-local-resource": "error",
    "unicorn/no-shorthand-property-overrides": "error",
    "unicorn/no-transition-all": "error",
    "unicorn/no-unnecessary-fetch-options": "error",
    "unicorn/no-url-in-search-params": "error",
    "unicorn/prefer-abort-signal-any": "error",
    "unicorn/prefer-abort-signal-timeout": "error",
    "unicorn/prefer-blob-reading-methods": "error",
    "unicorn/prefer-location-assign": "error",
    "unicorn/prefer-response-static-json": "error",
    "unicorn/prefer-url-can-parse": "error",
    "unicorn/prefer-url-href": "error",
    "unicorn/prefer-url-search-parameters": "error",
    "unicorn/relative-url-style": "error",
    "unicorn/require-css-escape": "error",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/require-frontmatter-fields": "off",
    "unicorn/require-post-message-target-origin": "error",
  },
};
