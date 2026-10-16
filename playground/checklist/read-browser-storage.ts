/**
 * Reads the browser's storage of the page, kept in this browser only. A browser blocking the site's
 * data throws on reaching it: none then, and the page goes on without it.
 *
 * @kind procedure
 * @param document - the playground's document
 * @returns the storage, or none when the browser gives none
 * @see docs/backlog/stories/E15-F17-US-051-validation-checklist-page.md
 */
export function readBrowserStorage(document: Document): Storage | undefined {
  const view = document.defaultView;

  if (view === null) {
    return undefined;
  }

  try {
    return view.localStorage;
  } catch {
    return undefined;
  }
}
