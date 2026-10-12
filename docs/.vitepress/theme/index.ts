import DefaultTheme from "vitepress/theme";

import { drawDiagrams } from "./draw-diagrams";

import type { Theme } from "vitepress";

/**
 * Theme of the documentation site: VitePress's default theme, with Mermaid diagrams drawn once the
 * page is in the browser, and again whenever its content changes (another page opened).
 *
 * @see docs/tooling/documentation-site.md
 */
const THEME: Theme = {
  enhanceApp: () => {
    if (typeof window === "undefined") {
      return;
    }

    const observer = new MutationObserver(() => {
      // eslint-disable-next-line @typescript-eslint/no-floating-promises -- a DOM observer cannot wait; drawDiagrams catches every error itself
      drawDiagrams();
    });

    observer.observe(document.body, { childList: true, subtree: true });
  },
  extends: DefaultTheme,
};

export default THEME;
