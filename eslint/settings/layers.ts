/**
 * Layered architecture: imports only go to lower layers (docs/conventions/architecture.md).
 *
 * `imports` lists the layers a layer may import; `kinds` lists the `@kind` values its
 * functions may carry. The playground only sees the public API (`src/index.ts`).
 */
export const LAYERS = [
  { name: "math", path: "src/math", imports: [], kinds: ["math"] },
  { name: "geometry", path: "src/geometry", imports: ["math"], kinds: ["math", "geometry"] },
  { name: "model", path: "src/model", imports: ["math", "geometry"], kinds: ["domain"] },
  {
    name: "routing",
    path: "src/routing",
    imports: ["math", "geometry", "model"],
    kinds: ["geometry", "domain"],
  },
  { name: "io", path: "src/io", imports: ["math", "geometry", "model"], kinds: ["format"] },
  {
    name: "render",
    path: "src/render",
    imports: ["math", "geometry", "model", "routing", "io"],
    kinds: ["format", "procedure"],
  },
  {
    name: "interaction",
    path: "src/interaction",
    imports: ["math", "geometry", "model", "routing", "io", "render"],
    kinds: ["domain", "procedure"],
  },
  { name: "playground", path: "playground", imports: [], kinds: ["procedure"] },
];

/** Library entry point: the only file the playground may import. */
const PUBLIC_API_PATH = "src/index.ts";

/** Library layers (every layer but the playground). */
const LIBRARY_LAYERS = LAYERS.filter((layer) => layer.path.startsWith("src/"));

/**
 * `import-x/no-restricted-paths` zones: a layer may only import the layers it lists.
 * The playground may only import the public API.
 */
export const LAYER_ZONES = [
  ...LIBRARY_LAYERS.map((layer) => ({
    from: LIBRARY_LAYERS.filter(
      (other) => other.name !== layer.name && !layer.imports.includes(other.name),
    ).map((other) => `./${other.path}`),
    message: `Layer \`${layer.name}\` may only import: ${layer.imports.length > 0 ? layer.imports.join(", ") : "nothing"}.`,
    target: `./${layer.path}`,
  })).filter((zone) => zone.from.length > 0),
  {
    except: [`./${PUBLIC_API_PATH.slice("src/".length)}`],
    from: "./src",
    message: "The playground only uses the public API (src/index.ts).",
    target: "./playground",
  },
];
