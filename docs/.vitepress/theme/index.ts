import DefaultTheme from "vitepress/theme";

import { drawDiagrams } from "./draw-diagrams";
import "./diagrams.css";

import type { Theme } from "vitepress";

/**
 * Theme of the documentation site: VitePress's default theme, with Mermaid diagrams drawn once the
 * page is in the browser, and again whenever its content changes (another page opened).
 *
 * @see docs/tooling/documentation-site.md
 */
const THEME: Theme = {
  enhanceApp: () => {
    if (!("window" in globalThis)) {
      return;
    }

    // Drawings run one after the other: Mermaid measures text in the page, and two drawings at
    // once disturb each other. The observer cannot wait, so each change queues the next one.
    const queue = { drawing: Promise.resolve() };
    const observer = new MutationObserver(() => {
      const previous = queue.drawing;

      queue.drawing = (async (): Promise<void> => {
        await previous;
        await drawDiagrams();
      })();
    });

    observer.observe(document.body, { childList: true, subtree: true });
  },
  extends: DefaultTheme,
};

export default THEME;
