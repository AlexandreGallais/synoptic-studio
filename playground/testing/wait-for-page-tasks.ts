// lint-disable-next-line local/canonical-import-path -- the rule names the first folder under the common ancestor, playground/src, which has no barrel in the Angular app
import { PENDING_TASKS } from "../src/app/shared/utils/page";

/**
 * Reads the tasks the page started.
 *
 * @kind procedure
 * @returns them, in the order they started; none before the first
 */
function readPageTasks(): readonly Promise<void>[] {
  return PENDING_TASKS.get(document) ?? [];
}

/**
 * Waits, as a user waits for the page, until every task it started is done, those the tasks
 * started included (`PENDING_TASKS`).
 *
 * @kind procedure
 * @returns once no task is left running
 */
export async function waitForPageTasks(): Promise<void> {
  const tasks = readPageTasks();

  await Promise.all(tasks);

  if (readPageTasks().length > tasks.length) {
    await waitForPageTasks();
  }
}
