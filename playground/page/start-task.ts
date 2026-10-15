/**
 * Starts an asynchronous task from an event listener, which returns nothing and cannot await it:
 * a file read, the fonts the browser lists (US-036). Its failure is handed to `onFailure`, never
 * left unhandled.
 *
 * @kind procedure
 * @param task - the task to start
 * @param onFailure - called with the error when the task fails
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function startTask(task: () => Promise<void>, onFailure: (error: unknown) => void): void {
  // eslint-disable-next-line unicorn/prefer-await -- an event listener cannot await: the one place a task is started, its failure handled
  task().catch(onFailure);
}
