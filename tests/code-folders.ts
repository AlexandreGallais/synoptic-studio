// The folders whose plain TypeScript the repository's tests read (EN-175).

/**
 * The folders of plain TypeScript under the library's rules, from the repository's root: the
 * library; the old page's folders, kept unchanged as the playground's utils (EN-160); the helpers
 * of the playground's tests. The rest of its Angular app follows Angular's usage (ADR-0104) and
 * is left to the playground's own lanes.
 */
export const CODE_FOLDERS = ["src", "playground/src/app/shared/utils", "playground/testing"];
