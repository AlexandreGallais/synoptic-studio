import type { TSESLint } from "@typescript-eslint/utils";

/** Classes and `this` (classes are banned by functional/no-classes; these rules guard the remainder). */
export const CORE_CLASSES: TSESLint.FlatConfig.Config = {
  name: "core/classes",
  rules: {
    // Superseded by the type-aware @typescript-eslint/class-methods-use-this.
    "class-methods-use-this": "off",
    "consistent-this": ["error", "self"],
    "constructor-super": "error",
    "new-cap": ["error", { capIsNew: true, newIsCap: true, properties: true }],
    "no-class-assign": "error",
    "no-constructor-return": "error",
    // Superseded by the type-aware @typescript-eslint/no-dupe-class-members.
    "no-dupe-class-members": "off",
    "no-empty-static-block": "error",
    // Superseded by the type-aware @typescript-eslint/no-invalid-this.
    "no-invalid-this": "off",
    "no-new": "error",
    "no-new-native-nonconstructor": "error",
    "no-this-before-super": "error",
    // Superseded by the type-aware @typescript-eslint/no-unused-private-class-members.
    "no-unused-private-class-members": "off",
    // Superseded by the type-aware @typescript-eslint/no-useless-constructor.
    "no-useless-constructor": "off",
  },
};
