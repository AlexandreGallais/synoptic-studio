// A value slow to prepare, built at its first use and shared after it (EN-089, EN-090).

/**
 * A function that builds its value at its first call and gives that same value afterwards: a
 * test file that never asks for it never builds it, and the files of a worker that shares its
 * modules (`isolate: false`, ADR-0070) build it once between them. The value is frozen by its
 * builder, so that no test changes it for the next.
 *
 * @template T - the value's type
 * @param build - builds the value
 * @returns the value, built once
 */
export function builtOnce<T>(build: () => T): () => T {
  const built: T[] = [];

  return () => {
    if (built.length === 0) {
      built.push(build());
    }

    return built[0] ?? build();
  };
}
