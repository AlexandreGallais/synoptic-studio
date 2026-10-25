/**
 * Reads the browser's storage of the page, kept in this browser only. A browser blocking the site's
 * data throws on reaching it: none then, and the page goes on without it.
 *
 * @kind procedure
 * @param document - the playground's document
 * @returns the storage, or none when the browser gives none
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
