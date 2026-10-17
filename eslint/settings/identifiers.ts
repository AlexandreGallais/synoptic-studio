/** Single-letter names allowed: standard mathematical variables (x, y, r, θ written `t`…). */
export const SHORT_MATH_IDENTIFIERS = [
  "a",
  "b",
  "c",
  "d",
  "h",
  "i",
  "j",
  "k",
  "n",
  "p",
  "q",
  "r",
  "s",
  "t",
  "u",
  "v",
  "w",
  "x",
  "y",
  "z",
];

/** Names too vague to state a concept. */
export const DENIED_IDENTIFIERS = [
  "bar",
  "baz",
  "foo",
  "helper",
  "helpers",
  "info",
  "manager",
  "obj",
  "res",
  "ret",
  "stuff",
  "temp",
  "thing",
  "tmp",
  "util",
  "utils",
  "val",
];

/** Browser globals easily used by mistake, and legacy global functions. */
export const RESTRICTED_GLOBALS = [
  { message: "Use Number.isFinite.", name: "isFinite" },
  { message: "Use Number.isNaN.", name: "isNaN" },
  { message: "Use the event parameter of the listener.", name: "event" },
  { message: "Ambiguous browser global (window.name).", name: "name" },
  { message: "Ambiguous browser global (window.length).", name: "length" },
  { message: "Ambiguous browser global (window.status).", name: "status" },
  { message: "Ambiguous browser global (window.parent).", name: "parent" },
  { message: "Ambiguous browser global (window.top).", name: "top" },
];

/**
 * Globals of a host (browser or Node.js), forbidden in the functional core: it runs in any
 * ECMAScript engine and stays portable to another language (ADR-0025).
 */
export const DOM_GLOBALS = [
  "Buffer",
  "Intl",
  "__dirname",
  "__filename",
  "console",
  "crypto",
  "customElements",
  "document",
  "fetch",
  "global",
  "globalThis",
  "localStorage",
  "location",
  "module",
  "navigator",
  "process",
  "queueMicrotask",
  "require",
  "requestAnimationFrame",
  "sessionStorage",
  "setInterval",
  "setTimeout",
  "structuredClone",
  "window",
].map((name) => ({
  message: "The functional core uses ECMAScript only: no DOM, no Node.js, no host API (ADR-0025).",
  name,
}));

/** Non-deterministic properties: inputs are passed explicitly instead. */
export const RESTRICTED_PROPERTIES = [
  {
    message: "Non-deterministic: pass the value as an argument.",
    object: "Math",
    property: "random",
  },
  { message: "Non-deterministic: pass the value as an argument.", object: "Date", property: "now" },
  {
    message: "Non-deterministic: pass the value as an argument.",
    object: "performance",
    property: "now",
  },
];

/** Mutable collection types replaced by their read-only counterparts. */
export const RESTRICTED_TYPES = {
  Map: { fixWith: "ReadonlyMap", message: "Data is immutable: use ReadonlyMap." },
  Set: { fixWith: "ReadonlySet", message: "Data is immutable: use ReadonlySet." },
};
