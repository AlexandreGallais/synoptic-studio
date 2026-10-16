import { PENDING_TASKS } from "./pending-tasks";

/**
 * Runs a task, its failure handed to `onFailure`.
 *
 * @kind procedure
 * @param task - the work to run
 * @param onFailure - called with the error when the task fails
 * @returns once the task is done or its failure handled
 * @see docs/backlog/stories/E15-F15-US-036-import-choose-and-embed-a-font.md
 */
async function runSettled(
  task: () => Promise<void>,
  onFailure: (error: unknown) => void,
): Promise<void> {
  try {
    await task();
  } catch (error: unknown) {
    onFailure(error);
  }
}

/**
 * Runs an asynchronous task started by an event listener, which returns nothing and cannot await
 * it — a font file read, the fonts the browser lists (US-036): the task's promise is kept with the
 * page's pending tasks (`PENDING_TASKS`), never left floating, and its failure handed to
 * `onFailure`.
 *
 * @kind procedure
 * @param document - page document
 * @param task - the task to run
 * @param onFailure - called with the error when the task fails
 * @see docs/backlog/stories/E15-F15-US-036-import-choose-and-embed-a-font.md
 */
export function runTask(
  document: Document,
  task: () => Promise<void>,
  onFailure: (error: unknown) => void,
): void {
  const running = runSettled(task, onFailure);

  PENDING_TASKS.set(document, [...(PENDING_TASKS.get(document) ?? []), running]);
}
