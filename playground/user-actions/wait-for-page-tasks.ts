import { PENDING_TASKS } from "../page";

/**
 * Reads the tasks the page started.
 *
 * @kind procedure
 * @returns them, in the order they started; none before the first
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
function readPageTasks(): readonly Promise<void>[] {
  return PENDING_TASKS.get(document) ?? [];
}

/**
 * Waits, as a user waits for the page, until every task it started is done, those the tasks
 * started included (`PENDING_TASKS`, US-036).
 *
 * @kind procedure
 * @returns once no task is left running
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export async function waitForPageTasks(): Promise<void> {
  const tasks = readPageTasks();

  await Promise.all(tasks);

  if (readPageTasks().length > tasks.length) {
    await waitForPageTasks();
  }
}
