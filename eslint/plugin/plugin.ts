import {
  BARREL_EXPORTS_RULE,
  CANONICAL_IMPORT_PATH_RULE,
  FOLDER_HAS_INDEX_RULE,
  KIND_IN_LAYER_RULE,
  KIND_LIMITS_RULE,
  KIND_NAMING_RULE,
  ONE_EXPORT_PER_FILE_RULE,
  REQUIRE_KIND_RULE,
  SEE_REFERENCES_RULE,
} from "./rules";

/**
 * Project-specific ESLint plugin, registered under the `local` prefix.
 *
 * @see docs/conventions/lint.md
 */
export const LOCAL_PLUGIN = {
  meta: { name: "local" },
  rules: {
    "barrel-exports": BARREL_EXPORTS_RULE,
    "canonical-import-path": CANONICAL_IMPORT_PATH_RULE,
    "folder-has-index": FOLDER_HAS_INDEX_RULE,
    "kind-in-layer": KIND_IN_LAYER_RULE,
    "kind-limits": KIND_LIMITS_RULE,
    "kind-naming": KIND_NAMING_RULE,
    "one-export-per-file": ONE_EXPORT_PER_FILE_RULE,
    "require-kind": REQUIRE_KIND_RULE,
    "see-references": SEE_REFERENCES_RULE,
  },
};
