/**
 * Yields to the page between two pieces of a long work, as web.dev advises without
 * `scheduler.yield()`: a promise resolved by `setTimeout(resolve, 0)`, which runs in a task of
 * its own, after those waiting, so that the page draws and answers clicks meanwhile
 * (`REF-WEBDEV-LONG-TASKS`).
 *
 * @kind procedure
 * @param view - the page's window
 * @returns once the page has run the tasks that were waiting
 */
export async function waitForNextTask(view: Pick<Window, "setTimeout">): Promise<void> {
  await new Promise<void>((resolve) => {
    view.setTimeout(resolve, 0);
  });
}
