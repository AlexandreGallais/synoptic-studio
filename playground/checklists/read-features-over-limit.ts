import type { ChecklistLine } from "./checklist-line";

/**
 * The features of a checklist holding more lines than a limit, each named with its count
 * (`F05: 14 lines`), in the order they first appear; empty when every feature fits (EN-056).
 *
 * @kind procedure
 * @param lines - the checklist's lines, each with its feature
 * @param limit - the most lines a feature may hold, 12
 * @returns each feature over the limit with its count
 */
export function readFeaturesOverLimit(
  lines: readonly ChecklistLine[],
  limit: number,
): readonly string[] {
  const features = [...new Set(lines.map((line) => line.feature))];

  return features
    .map((feature) => ({
      count: lines.filter((line) => line.feature === feature).length,
      feature,
    }))
    .filter(({ count }) => count > limit)
    .map(({ count, feature }) => `${feature}: ${String(count)} lines`);
}
