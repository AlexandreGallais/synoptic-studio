import markdown from "@eslint/markdown";

import { MARKDOWN_FILES } from "../settings";

import type { Linter } from "eslint";

/** Markdown pages: GitHub-flavored Markdown with YAML front matter. */
export const MARKDOWN_LANGUAGE: Linter.Config = {
  files: MARKDOWN_FILES,
  language: "markdown/gfm",
  languageOptions: { frontmatter: "yaml" },
  name: "scopes/markdown-language",
  plugins: { markdown },
};
