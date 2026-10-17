//! The probe files EN-080 compared with `ESLint`: each one's text, and the place and the rule of
//! every finding `ESLint` gave on it with the 33 rules carried here, the project's options on.
//! The Rust linter must give the same.

use crate::rules::unicorn::RULES;
use crate::testing::lint;

/// The rules of EN-080.
const EN_080: [&str; 33] = [
    "unicorn/consistent-destructuring",
    "unicorn/no-computed-property-existence-check",
    "unicorn/no-invalid-property-descriptor",
    "unicorn/no-invalid-well-known-symbol-methods",
    "unicorn/no-unreadable-new-expression",
    "unicorn/no-unreadable-object-destructuring",
    "unicorn/no-unsafe-property-key",
    "unicorn/no-unused-builtin-method-return",
    "unicorn/no-unused-properties",
    "unicorn/prefer-object-define-properties",
    "unicorn/prefer-object-destructuring-defaults",
    "unicorn/prefer-single-object-destructuring",
    "unicorn/no-incorrect-template-string-interpolation",
    "unicorn/no-unnecessary-string-trim",
    "unicorn/no-unsafe-string-replacement",
    "unicorn/prefer-literal-ascii",
    "unicorn/prefer-regexp-escape",
    "unicorn/prefer-short-escape-sequences",
    "unicorn/prefer-single-replace",
    "unicorn/prefer-split-limit",
    "unicorn/prefer-string-pad-start-end",
    "unicorn/prefer-string-repeat",
    "unicorn/prefer-unicode-code-point-escapes",
    "unicorn/comment-content",
    "unicorn/consistent-compound-words",
    "unicorn/consistent-tuple-labels",
    "unicorn/name-replacements",
    "unicorn/no-keyword-prefix",
    "unicorn/prefer-type-literal-last",
    "unicorn/consistent-optional-chaining",
    "unicorn/no-useless-compound-assignment",
    "unicorn/no-useless-delete-check",
    "unicorn/prefer-https",
];

/// A finding's place and rule: its line, its column, the rule's name.
type Place = (usize, usize, &'static str);

/// Each probe: its name, its text, and the findings `ESLint` gave on it.
const PROBES: [(&str, &str, &[Place]); 23] = [
    (
        "cd-more.ts",
        r"declare const foo: any, other: any, update: any, k: string;
{ const {a} = foo; console.log(foo.a.b, foo?.a); }
{ const {a} = foo; foo.a++; console.log(foo.a); }
{ const {a} = foo; delete foo.a; console.log(foo.a); }
{ const {a} = foo; [foo.a] = [1]; console.log(foo.a); }
{ const {a} = foo; ({x: foo.a} = other); console.log(foo.a); }
{ const {a} = foo; ({x: foo.a = 1} = other); console.log(foo.a); }
{ const {a} = foo; for (foo.a of []) {} console.log(foo.a); }
{ const {a} = foo; for (foo.a in {}) {} console.log(foo.a); }
{ const {a} = foo; const g = (): void => { foo.a = 2; }; console.log(g, foo.a); }
{ const {a} = foo; new update(); console.log(foo.a); }
{ const {a} = foo; update`x`; console.log(foo.a); }
{ const {a} = foo; foo.b.update(); console.log(foo.a); }
{ const {a} = foo; const h = (): unknown => foo.a; console.log(h); }
{ const {a} = foo; if ('a' in foo) { console.log(foo.a); } }
{ const {a} = foo; const v = 'b' in foo ? foo.a : 0; console.log(v); }
{ const {a} = foo; const w = ok() && foo.a; console.log(w); }
{ const {a} = (foo as any); console.log(foo.a); }
{ const {a} = foo; console.log((foo!).a, (foo as any).a, foo[k], foo.a /* c */); }
function ok(): boolean { return true; }
export function m(this: any): void { const {a} = this; console.log(this.a); }
export function n(this: any): void { const {a} = this; const i = function (this: any): void { console.log(this.a); }; console.log(i); }
{ const {a} = foo; { const foo = other; console.log(foo.a); } }
{ let bar = foo; const {a} = bar; bar = other; console.log(bar.a); }
{ let bar = foo; const {a} = bar; const j = (): void => { bar = other; }; console.log(j, bar.a); }
",
        &[
            (6, 25, "unicorn/no-unreadable-object-destructuring"),
            (7, 25, "unicorn/no-unreadable-object-destructuring"),
            (21, 68, "unicorn/consistent-destructuring"),
            (22, 62, "unicorn/name-replacements"),
            (25, 41, "unicorn/name-replacements"),
        ],
    ),
    (
        "consistent-destructuring.ts",
        r"declare const foo: any, update: any, k: string, ok: boolean;
{ const {a} = foo; console.log(foo.a); }
{ const {a} = foo; console.log(a); }
{ console.log(foo.a, foo.b); }
{ const {a} = foo; console.log(a, foo.b); }
{ const { a: {b} } = foo; console.log(foo.a.c, b); }
{ const {bar} = foo; const {a} = foo.bar; console.log(bar, a); }
{ const {bar} = foo; const {a} = bar; console.log(a); }
{ const {a} = foo; console.log(a, foo.b()); }
{ const {a} = foo.bar; console.log(foo.bar.a, a); }
{ const {a} = foo; update(foo); console.log(foo.a, a); }
{ const {a} = foo; foo.update(); console.log(foo.a, a); }
{ const {a: b} = foo; console.log(foo.a, b); }
{ const {a} = foo; console.log((foo as any).a, a); }
{ const {a} = foo; console.log(foo.a.b, a); }
{ const {a} = foo; function f() { return foo.a; } console.log(f, a); }
{ const {a} = foo; foo.a = 1; console.log(a); }
{ const {a} = foo; foo.a(); console.log(a); }
{ const {a} = foo; foo.a = 2; console.log(foo.a, a); }
{ let {a} = foo; console.log(foo.a, a); a = 1; }
{ const {a = 1} = foo; console.log(foo.a, a); }
{ const {[k]: a} = foo; console.log(foo.a, a); }
{ console.log(foo.a); const {a} = foo; console.log(a); }
{ const {a} = foo; function f(a: any) { return foo.a + a; } console.log(f); }
{ const {a} = foo; const x = 'a' in foo ? foo.a : 0; console.log(x, a); }
{ const {a} = foo; if ('a' in foo && ok) { console.log(foo.a, a); } }
{ const {a} = foo; const g = () => foo.a; update(); console.log(g, foo.a, a); }
{ const {a} = foo; const h = () => update(); console.log(h, foo.a, a); }
{ const {a, b} = foo; console.log(foo.a + foo.b, a, b); }
{ const {a} = foo; const z = foo.a!; console.log(z, a); }
{ const {a} = foo; const z = foo?.a; console.log(z, a); }
",
        &[
            (2, 32, "unicorn/consistent-destructuring"),
            (7, 34, "unicorn/consistent-destructuring"),
            (13, 35, "unicorn/consistent-destructuring"),
            (14, 32, "unicorn/consistent-destructuring"),
            (15, 32, "unicorn/consistent-destructuring"),
            (22, 10, "unicorn/no-unreadable-object-destructuring"),
            (28, 61, "unicorn/consistent-destructuring"),
            (29, 35, "unicorn/consistent-destructuring"),
            (29, 43, "unicorn/consistent-destructuring"),
            (30, 30, "unicorn/consistent-destructuring"),
            (31, 30, "unicorn/consistent-destructuring"),
        ],
    ),
    (
        "define-properties.ts",
        r"declare const foo: object, other: object, o: object, d: PropertyDescriptor, e: PropertyDescriptor, g: PropertyDescriptor, key: string, k: string;
declare function f(): void;
Object.defineProperty(foo, 'bar', {
  value: 1,
  writable: true,
});
Object.defineProperty(foo, 'baz', {
  value: 2,
  writable: true,
});
export function f1(): void {
  Object.defineProperty(o, 'a-b', d);
  Object.defineProperty(o, 0, e);
}
export function f2(): void {
  Object.defineProperty(o, 0, d);
  Object.defineProperty(o, key, e);
  Object.defineProperty(o, '__proto__', g);
}
Object.defineProperty(other, 'bar', d);
Object.defineProperty(other, 'bar', e);
f();
Object.defineProperty(o, k, d);
Object.defineProperty(o, k, e);
f();
Object.defineProperty(foo, 'a', d); // note
Object.defineProperty(foo, 'b', e);
f();
Object.defineProperty(foo, 'a', {value: `x
y`});
Object.defineProperty(foo, 'b', e);
f();
Object.defineProperty(foo, 'bar', d);
Object.defineProperty(other, 'baz', e);
Reflect.defineProperty(foo, 'bar', d);
Reflect.defineProperty(foo, 'baz', e);
export function f3(): void {
  if (o) {
    Object.defineProperty(o, 'x', {
      get() { return 1; },
    });
    Object.defineProperty(o, Symbol.iterator, d);
    Object.defineProperty(o, 'y', e);
  }
}
",
        &[
            (1, 77, "unicorn/name-replacements"),
            (3, 8, "unicorn/prefer-object-define-properties"),
            (12, 10, "unicorn/prefer-object-define-properties"),
            (16, 10, "unicorn/prefer-object-define-properties"),
            (20, 8, "unicorn/prefer-object-define-properties"),
            (23, 8, "unicorn/prefer-object-define-properties"),
            (26, 8, "unicorn/prefer-object-define-properties"),
            (29, 8, "unicorn/prefer-object-define-properties"),
            (39, 12, "unicorn/prefer-object-define-properties"),
        ],
    ),
    (
        "destructuring.ts",
        r"declare const options: object, k: string, x: number, a: { b: number }, o: object;
declare function f(): number; declare function g(): object; declare function getFoo(): { bar: number; baz: number }; declare function use(v: unknown): void;
{ const {foo, bar} = { foo: false, bar: 1, ...options }; use([foo, bar]); }
{ const {foo: localFoo} = { foo: false, ...options }; use(localFoo); }
{ const {a1} = {a1: `x`, ...o}; use(a1); }
{ const {a2} = {a2: -1, ...o}; use(a2); }
{ const {foo = false, bar = 1} = {...options}; use([foo, bar]); }
{ const {foo} = {foo: false, bar: 1, ...options}; use(foo); }
{ const {foo, ...rest} = {foo: false, ...options}; use([foo, rest]); }
{ const {foo} = {...options, foo: false}; use(foo); }
{ const {foo} = {foo: f(), ...options}; use(foo); }
{ const {foo} = {foo: a.b, ...options}; use(foo); }
{ const {foo} = {foo: false, ...g()}; use(foo); }
{ const {toString} = {toString: 1, ...options}; use(toString); }
{ const {foo} = {foo: `${x}`, ...options}; use(foo); }
{ const {foo} = {foo: x + 1, ...options}; use(foo); }
{ const {foo} = {foo: 1 + 1, ...options}; use(foo); }
{ const {foo} = {foo: [1, x], ...options}; use(foo); }
{ const foo = {bar: 1, baz: 2}; const {bar} = foo; const {baz} = foo; use([bar, baz]); }
{ const foo = getFoo(); const {bar, baz} = foo; use([bar, baz]); }
export async function f1(): Promise<void> { const p = await getFoo(); let {bar} = p; use(bar); bar = 1; }
{ const foo = getFoo(); const {bar} = foo; // first
  const {baz} = foo; use([bar, baz]); }
{ let foo = getFoo(); const {bar} = foo; const {baz} = foo; use([bar, baz]); foo = getFoo(); }
{ const foo = getFoo(); const {bar} = foo; use(bar); const {baz} = foo; use(baz); }
{ const foo = getFoo(); const {bar} = foo; const {bar: b} = foo; use([bar, b]); }
{ const foo = getFoo(); const {bar} = foo; use([bar, foo]); }
{ const foo: { bar: number } = getFoo(); const {bar} = foo; use(bar); }
{ const foo = (): number => 1; const {name} = foo; use(name); }
{ const foo = getFoo(); const {bar} = foo; const {baz} = foo; const {bar: c} = foo; use([bar, baz, c]); }
",
        &[
            (3, 9, "unicorn/prefer-object-destructuring-defaults"),
            (4, 9, "unicorn/prefer-object-destructuring-defaults"),
            (5, 9, "unicorn/prefer-object-destructuring-defaults"),
            (6, 9, "unicorn/prefer-object-destructuring-defaults"),
            (15, 9, "unicorn/prefer-object-destructuring-defaults"),
            (17, 9, "unicorn/prefer-object-destructuring-defaults"),
            (18, 9, "unicorn/prefer-object-destructuring-defaults"),
            (19, 52, "unicorn/prefer-single-object-destructuring"),
            (20, 9, "unicorn/prefer-single-object-destructuring"),
            (21, 51, "unicorn/prefer-single-object-destructuring"),
            (23, 3, "unicorn/prefer-single-object-destructuring"),
            (30, 44, "unicorn/prefer-single-object-destructuring"),
            (30, 63, "unicorn/prefer-single-object-destructuring"),
        ],
    ),
    (
        "escapes.ts",
        r#"declare const x: string, bar: string;
export const letter = '\x41';
export const word = '\u0042\u{43}';
export const template = `\u0024\u007Bvalue}`;
export const q1 = '\x27';
export const q2 = "\x27";
export const b1 = '\x5C';
export const t1 = `\x60${x}\x41`;
export const o1 = '\0\x31';
export const n1 = 'A';
export const n2 = '\n\t\u00A9\x7F\x1F';
export const n3 = '\\x41';
export const r1 = String.raw`\x41`;
export const r2 = /\x41/u;
export const text = "Line one\u000ALine two";
export const tt = `Line one\u000ALine two`;
export const sx = '\u0009\u000b\u0000x\u0008\u000C\u000D';
export const sy = `${bar}\u000a`;
export const s1 = '\\u000A';
export const s2 = '\u00000';
export const s3 = '\u{A}';
export const s4 = '\x0A';
export const s5 = String.raw`\u000A`;
export const u1 = '\xA9';
export const u2 = '\u2661';
export const u3 = '\uD83D\uDCA9';
export const u4 = `\xA9${bar}\u2661`;
export const u5 = /\u0061/u;
export const u6 = /\cJ[\uD83D]/u;
export const u7 = /\u0061/;
export const u8 = /\u{61}/;
export const u9 = '\u{A9}';
export const u10 = /\u{61}/u;
export const u11 = '\u0022\u0027\u002F\u005C\u0060';
export const u12 = /[\uD83D\uDCA9]\uD83D\uDCA9/u;
export const u13 = /[[\xA9]]/v;
type T = 'a\x41';
export const t: T = 'aA';
export const k = { '\x41': 1, '\u00e9': 2 };
export const m = `a\u0060b\x24{c}`;
"#,
        &[
            (2, 23, "unicorn/prefer-literal-ascii"),
            (3, 21, "unicorn/prefer-literal-ascii"),
            (4, 25, "unicorn/prefer-literal-ascii"),
            (5, 19, "unicorn/prefer-literal-ascii"),
            (6, 19, "unicorn/prefer-literal-ascii"),
            (7, 19, "unicorn/prefer-literal-ascii"),
            (8, 19, "unicorn/prefer-literal-ascii"),
            (8, 27, "unicorn/prefer-literal-ascii"),
            (9, 19, "unicorn/prefer-literal-ascii"),
            (11, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (14, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (15, 21, "unicorn/prefer-short-escape-sequences"),
            (16, 19, "unicorn/prefer-short-escape-sequences"),
            (17, 19, "unicorn/prefer-short-escape-sequences"),
            (18, 25, "unicorn/prefer-short-escape-sequences"),
            (20, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (22, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (24, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (25, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (26, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (27, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (27, 29, "unicorn/prefer-unicode-code-point-escapes"),
            (28, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (29, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (30, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (31, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (34, 20, "unicorn/prefer-literal-ascii"),
            (35, 20, "unicorn/prefer-unicode-code-point-escapes"),
            (36, 20, "unicorn/prefer-unicode-code-point-escapes"),
            (37, 10, "unicorn/prefer-literal-ascii"),
            (39, 20, "unicorn/prefer-literal-ascii"),
            (39, 31, "unicorn/prefer-unicode-code-point-escapes"),
            (40, 18, "unicorn/prefer-literal-ascii"),
            (40, 31, "unicorn/no-incorrect-template-string-interpolation"),
        ],
    ),
    (
        "misc-more.ts",
        r#"declare const foo: any, s: string, k: string, f: any, list: number[];
export const c1 = this?.a || this.b;
export const c2 = foo.a.b?.c && foo.a.b?.d;
export const c3 = foo[k]?.x || foo[k].y;
export const c4 = (foo?.a as any) || foo.b;
export const c5 = foo?.a! || foo.b;
export const c6 = f()?.a || f().b;
export const c7 = foo?.a.b || foo.c;
export const c8 = foo?.[0] || foo[1];
export const e1 = s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
export const e2 = s.replace(/[.*+?^${}()|[\]\\-]/g, '\\$&');
export const e3 = s.replace(/[.*+?^${}()|[\]\\a]/g, '\\$&');
export const e5 = s.replace(/[.*+?^${}()|[-\]\\]/g, '\\$&');
export const e6 = s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&', 1);
export const e7 = s?.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
export const e8 = list.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
export const e9 = s.replace(/[.*+?^${}()|[\]\\]/g, /* c */ '\\$&');
export const e10 = s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
export const e11 = (s + 'x').replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
export const e12 = s.replace(/[\.\*\+\?\^\$\{\}\(\)\|\[\]\\]/g, '\\$&');
export const e13 = s.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
"#,
        &[
            (2, 30, "unicorn/consistent-optional-chaining"),
            (3, 33, "unicorn/consistent-optional-chaining"),
            (4, 32, "unicorn/consistent-optional-chaining"),
            (5, 38, "unicorn/consistent-optional-chaining"),
            (6, 30, "unicorn/consistent-optional-chaining"),
            (9, 31, "unicorn/consistent-optional-chaining"),
            (10, 14, "unicorn/name-replacements"),
            (10, 19, "unicorn/prefer-regexp-escape"),
            (11, 14, "unicorn/name-replacements"),
            (11, 19, "unicorn/prefer-regexp-escape"),
            (12, 14, "unicorn/name-replacements"),
            (13, 14, "unicorn/name-replacements"),
            (14, 14, "unicorn/name-replacements"),
            (15, 14, "unicorn/name-replacements"),
            (16, 14, "unicorn/name-replacements"),
            (17, 14, "unicorn/name-replacements"),
            (17, 19, "unicorn/prefer-regexp-escape"),
            (18, 14, "unicorn/name-replacements"),
            (18, 20, "unicorn/prefer-regexp-escape"),
            (19, 14, "unicorn/name-replacements"),
            (19, 20, "unicorn/prefer-regexp-escape"),
            (20, 14, "unicorn/name-replacements"),
            (20, 20, "unicorn/prefer-regexp-escape"),
            (21, 14, "unicorn/name-replacements"),
            (21, 20, "unicorn/prefer-regexp-escape"),
        ],
    ),
    (
        "misc.ts",
        r#"declare const foo: any, a: any, other: any, ok: boolean, key: string, value: number, getTotal: () => number, handleMissingKey: () => void, k: string, f: () => void;
export const o1 = foo?.bar || foo.baz;
export const o2 = foo.bar || foo?.baz;
export const o3 = foo?.bar && foo?.baz;
export const o4 = foo.bar && foo?.baz;
export const o5 = a.b?.c || a.b.d;
export const o6 = foo?.bar || foo?.baz;
export const o7 = foo?.bar && foo.baz;
export const o8 = foo && foo.bar;
export const o9 = foo?.bar ?? foo.baz;
export const o10 = foo?.[key] || foo[key];
export type Point = [x: number, number];
export type Route = [name: string, ...string[]];
export type T3 = [number, y: number, string];
export type P2 = [x: number, y: number];
export type R2 = [name: string, ...segments: string[]];
export const newFoo = 'foo';
export const classFoo = 'foo';
export function newThing(): void {}
{ const {a: newA} = foo; console.log(newA); }
export const _newFoo = 'foo';
export const new_foo = 'foo';
export const fooNew = 'foo';
export const news = 1;
export const object = {newFoo: 1};
foo.newFoo = 2;
{ const {newFoo: x1} = foo; console.log(x1); }
{ const {newBar} = foo; console.log(newBar); }
console.log(foo.newFoo);
export function g(newBaz = 1): number { return newBaz; }
export type NewT = 1; export type X = newType; type newType = 2;
newLabel: for (;;) { break newLabel; }
{ let total = getTotal(); total += 0; total *= 1; total -= 0; total /= 1.0; total **= 1; total += 1; total |= 0; console.log(total); }
export function h(count: number): number { return count -= 0; }
{ const map = new Map<string, number>(); if (map.has(key)) { map.delete(key); } }
{ const set = new Set<number>(); if (set.has(value)) { set.delete(value); } }
{ const map = new Map<string, number>(); if (map.has(key)) { map.delete(key); } else { handleMissingKey(); } }
{ const record = {} as Record<string, number>; if (key in record) { delete record[key]; } }
{ const s = new WeakSet<object>(); if (s.has(foo)) s.delete(foo) }
{ const o = {a: 1} as {a?: number}; if ('a' in o) delete o.a; }
{ const m = new Map<string, number>(); if (m.has(k)) { // keep
  m.delete(k); } }
export type U1 = { foo: string } | Other;
export type U2 = Other | { foo: string };
export type U3 = Other | { foo: string } | undefined;
export type U4 = undefined | {a: 1} | Other | null | {b: 2};
export type U5 = {a: 1} /* first */ | Other;
export type U6 = {a: 1} | (Other & {b: 1});
type Other = { x: 1 };
export const u1 = 'http://sindresorhus.com';
// See http://sindresorhus.com
export const u2 = `x http://user@a.b.co:80/p?q=1 y`;
export const u3 = 'https://sindresorhus.com';
export const u4 = 'http://localhost:5173';
export const u5 = 'http://127.0.0.1';
export const u6 = 'http://www.w3.org/2000/svg';
export const u7 = 'http://site.test/a';
export const u8 = '<svg xmlns="http://other.org/ns">';
export const u9 = 'HTTP://SITE.COM';
export const u10 = 'http://www.w3.org/2000/09/xmldsig#rsa-sha1';
export const u11 = 'see http://example.com/a, and http://foo.bar.';
export const u12 = 'http://a.b:99999/x http://a^b.com http://[::1]/x http://xn--d1acufc.xn--p1ai';
// nodejs uses javascript.
// See the github issue.
// The application stores png files.
/** Draws an svg path, see `svg` and https://svg.org and package.json. */
export const e = new Error();
export class Btn {}
export const passWord = 'secret';
export const isInViewPort = true;
export function unSubscribe(): void {}
"#,
        &[
            (2, 31, "unicorn/consistent-optional-chaining"),
            (3, 30, "unicorn/consistent-optional-chaining"),
            (4, 31, "unicorn/consistent-optional-chaining"),
            (5, 30, "unicorn/consistent-optional-chaining"),
            (6, 29, "unicorn/consistent-optional-chaining"),
            (11, 34, "unicorn/consistent-optional-chaining"),
            (12, 33, "unicorn/consistent-tuple-labels"),
            (13, 36, "unicorn/consistent-tuple-labels"),
            (14, 19, "unicorn/consistent-tuple-labels"),
            (14, 38, "unicorn/consistent-tuple-labels"),
            (17, 14, "unicorn/no-keyword-prefix"),
            (18, 14, "unicorn/no-keyword-prefix"),
            (19, 17, "unicorn/no-keyword-prefix"),
            (20, 13, "unicorn/no-keyword-prefix"),
            (30, 48, "unicorn/no-keyword-prefix"),
            (31, 39, "unicorn/no-keyword-prefix"),
            (31, 53, "unicorn/no-keyword-prefix"),
            (32, 1, "unicorn/no-keyword-prefix"),
            (32, 28, "unicorn/no-keyword-prefix"),
            (33, 27, "unicorn/no-useless-compound-assignment"),
            (33, 39, "unicorn/no-useless-compound-assignment"),
            (33, 51, "unicorn/no-useless-compound-assignment"),
            (33, 63, "unicorn/no-useless-compound-assignment"),
            (33, 77, "unicorn/no-useless-compound-assignment"),
            (34, 51, "unicorn/no-useless-compound-assignment"),
            (35, 46, "unicorn/no-useless-delete-check"),
            (36, 38, "unicorn/no-useless-delete-check"),
            (38, 52, "unicorn/no-computed-property-existence-check"),
            (38, 52, "unicorn/no-useless-delete-check"),
            (39, 40, "unicorn/no-useless-delete-check"),
            (40, 41, "unicorn/no-useless-delete-check"),
            (41, 44, "unicorn/no-useless-delete-check"),
            (43, 18, "unicorn/prefer-type-literal-last"),
            (46, 30, "unicorn/prefer-type-literal-last"),
            (47, 18, "unicorn/prefer-type-literal-last"),
            (50, 20, "unicorn/prefer-https"),
            (51, 8, "unicorn/prefer-https"),
            (52, 22, "unicorn/prefer-https"),
            (61, 25, "unicorn/prefer-https"),
            (61, 51, "unicorn/prefer-https"),
            (62, 70, "unicorn/prefer-https"),
            (63, 4, "unicorn/comment-content"),
            (64, 12, "unicorn/comment-content"),
            (65, 8, "unicorn/comment-content"),
            (66, 14, "unicorn/comment-content"),
            (67, 14, "unicorn/name-replacements"),
            (68, 14, "unicorn/name-replacements"),
            (69, 14, "unicorn/consistent-compound-words"),
            (70, 14, "unicorn/consistent-compound-words"),
            (71, 17, "unicorn/consistent-compound-words"),
        ],
    ),
    (
        "naming-more.ts",
        r"import errDefault, * as utilsNs from './probe-helpers';
import { cbFn as cb, prop, e as evt } from './probe-helpers';
import pkgDefault from 'some-package';
declare function require(name: string): unknown;
const reqd = require('./x'); const reqPkg = require('pkg');
export { cb as callbackExport, prop };
export function f1(err: number, ...args: number[]): number { const { e = 1 } = { e: 2 }; return err + args.length + e; }
export const arrow = (idx: number, [el]: number[]): number => idx + el;
function f2(arguments_: number): number { const args = 1; return arguments_ + args; }
function f3(): number { const str = 'x'; const string_ = 1; return str.length + string_; }
const obj = { val: 1 }; const { val } = obj; let num = 1; num += val;
({ val: num } = obj); ({ num } = { num: 2 });
const errorCallback = 1; const errCallback = 2; const errorCb = 3;
type Props = { a: number }; interface Opts { b: number } type Obj<T> = T;
function f4<Val>(v: Val): Val { return v; }
class Btn { prop = 1; static cfg = 2; }
label1: for (const elem of [1]) { if (elem) break label1; }
try { f2(1); } catch (e) { console.log(e); }
export const newThing = 1, classThing = 2; export function newFn(newParam: number): number { return newParam; }
const { newA, b: newB, ['c']: newC, [k()]: newD } = { newA: 1, b: 2, c: 3 } as any;
({ newE: newA, f: num } = {} as any);
const [newF = 1] = [];
function k(): string { return 'x'; }
console.log(errDefault, utilsNs, cb, prop, evt, pkgDefault, reqd, reqPkg, f2, f3, num, errorCallback, errCallback, errorCb, f4, Btn, newA, newB, newC, newD, newF);
let x1: Props; let x2: Opts; let x3: Obj<number>; console.log(x1, x2, x3);
export const passWord = 1; export function onTimeOut(): void {} export const UserName = 2; export const metaDataStore = 3; export const subStringX = 4; export const workSpace_ = 5;
enum Enm { err = 1 } console.log(Enm.err);
",
        &[
            (1, 8, "unicorn/name-replacements"),
            (1, 25, "unicorn/name-replacements"),
            (2, 18, "unicorn/name-replacements"),
            (2, 22, "unicorn/name-replacements"),
            (2, 33, "unicorn/name-replacements"),
            (7, 20, "unicorn/name-replacements"),
            (7, 36, "unicorn/name-replacements"),
            (7, 70, "unicorn/name-replacements"),
            (8, 23, "unicorn/name-replacements"),
            (8, 37, "unicorn/name-replacements"),
            (9, 49, "unicorn/name-replacements"),
            (10, 31, "unicorn/name-replacements"),
            (11, 7, "unicorn/name-replacements"),
            (11, 50, "unicorn/name-replacements"),
            (13, 32, "unicorn/name-replacements"),
            (13, 55, "unicorn/name-replacements"),
            (14, 6, "unicorn/name-replacements"),
            (14, 39, "unicorn/name-replacements"),
            (14, 63, "unicorn/name-replacements"),
            (15, 13, "unicorn/name-replacements"),
            (16, 7, "unicorn/name-replacements"),
            (17, 20, "unicorn/name-replacements"),
            (18, 23, "unicorn/name-replacements"),
            (19, 14, "unicorn/no-keyword-prefix"),
            (19, 28, "unicorn/no-keyword-prefix"),
            (19, 60, "unicorn/name-replacements"),
            (19, 60, "unicorn/no-keyword-prefix"),
            (19, 66, "unicorn/name-replacements"),
            (19, 66, "unicorn/no-keyword-prefix"),
            (19, 101, "unicorn/no-keyword-prefix"),
            (20, 18, "unicorn/no-keyword-prefix"),
            (20, 24, "unicorn/no-unreadable-object-destructuring"),
            (20, 31, "unicorn/no-keyword-prefix"),
            (20, 37, "unicorn/no-unreadable-object-destructuring"),
            (20, 44, "unicorn/no-keyword-prefix"),
            (21, 10, "unicorn/no-keyword-prefix"),
            (26, 14, "unicorn/consistent-compound-words"),
            (26, 44, "unicorn/consistent-compound-words"),
            (26, 78, "unicorn/consistent-compound-words"),
            (26, 105, "unicorn/consistent-compound-words"),
            (26, 137, "unicorn/consistent-compound-words"),
            (26, 166, "unicorn/consistent-compound-words"),
        ],
    ),
    (
        "no-computed.ts",
        r"declare const object: any, key: string, ok: boolean, fallback: number, index: number;
if (object[key]) { console.log(1); }
if (key in object) { console.log(1); }
export const a1 = !object[key];
export const a2 = Boolean(object?.[key]);
while (ok && (object[key] as boolean)) { console.log(1); }
export const a3 = key in object;
if (object[key] === true) { console.log(1); }
if (Object.hasOwn(object, key)) { console.log(1); }
export const value = object[key];
if ('key' in object) { console.log(1); }
if (object['key']) { console.log(1); }
if (object[0]) { console.log(1); }
if (object[-1]) { console.log(1); }
if (object[`key`]) { console.log(1); }
const map = new Map<string, number>(); if (map[key as keyof typeof map]) { console.log(1); }
const set: Set<string> = new Set(); if (key in set) { console.log(1); }
if ('text'[index]) { console.log(1); }
export function f(text: string): void { if (text[index]) { console.log(1); } }
export const a4 = object[key] ?? fallback;
export const a5 = object[key] || fallback;
if (object[key]!) { console.log(1); }
export const a6 = ok ? object[key] : 0;
for (; object[key];) { console.log(1); }
const str = `abc`; if (str[index]) { console.log(1); }
if (object[key][key]) { console.log(1); }
",
        &[
            (2, 5, "unicorn/no-computed-property-existence-check"),
            (3, 5, "unicorn/no-computed-property-existence-check"),
            (4, 20, "unicorn/no-computed-property-existence-check"),
            (5, 27, "unicorn/no-computed-property-existence-check"),
            (6, 15, "unicorn/no-computed-property-existence-check"),
            (7, 19, "unicorn/no-computed-property-existence-check"),
            (22, 5, "unicorn/no-computed-property-existence-check"),
            (24, 8, "unicorn/no-computed-property-existence-check"),
            (25, 7, "unicorn/name-replacements"),
            (26, 5, "unicorn/no-computed-property-existence-check"),
        ],
    ),
    (
        "no-invalid-descriptor.ts",
        r"declare const object: object, getter: () => number, prototype: object, o: object, rest: object, descriptor: PropertyDescriptor, base: object;
Object.defineProperty(object, 'property', { get: getter, writable: false });
Reflect.defineProperty(object, 'property', {get: null});
// @ts-expect-error probe
Object.defineProperties(object, { property: {value: 1, writeable: true} });
// @ts-expect-error probe
Object.create(prototype, { property: 42 });
{ const descriptor1 = {get: getter, value: undefined}; Object.defineProperty(object, 'property', descriptor1); }
Object.defineProperty({}, 'property', { get: () => 1, writable: false } satisfies PropertyDescriptor);
// @ts-expect-error probe
Object.defineProperty(o, 'p', {[Symbol.iterator]: 1, set: 'x'});
// @ts-expect-error probe
Object.defineProperty(o, 'p', undefined);
Object.defineProperty(object, 'property', {get: getter});
Reflect.defineProperty(object, 'property', {get: undefined});
Object.defineProperties(object, { property: {value: 1, writable: true} });
Object.create(prototype, { property: {value: 42} });
{ const descriptor2 = {get: getter}; const descriptors = {property: descriptor2}; Object.create(prototype, descriptors); }
Object.defineProperty(o, 'p', {});
Object.defineProperty(o, 'p', {get: void 0, get set() { return () => 1; }});
Object.defineProperty(o, 'p', {...rest, writeable: 1});
Object.defineProperty(o, 'p', descriptor);
Object.freeze(o);
{ const d = {writeable: 1}; console.log(d.writeable); Object.defineProperty(o, 'p', d); }
export const d2 = {writeable: 1}; Object.defineProperty(o, 'p', d2);
Object.defineProperty(o, 'p', {__proto__: base, writeable: 1});
Object.defineProperty(o, 'p', {__proto__: null, writeable: 1, value: 1, get: 1n});
Object.defineProperty(o, 'p', 'text');
Object.defineProperty(o, 'p', `x`);
Object['defineProperty'](o, 'p', {foo: 1});
{ const d3 = {foo: 2}; Object.defineProperty(o, 'p', d3); Object.defineProperty(o, 'q', d3); }
{ const m = {a: {bar: 1}, b: 3}; Object.defineProperties(o, m); }
",
        &[
            (2, 43, "unicorn/no-invalid-property-descriptor"),
            (3, 50, "unicorn/no-invalid-property-descriptor"),
            (5, 56, "unicorn/no-invalid-property-descriptor"),
            (7, 38, "unicorn/no-invalid-property-descriptor"),
            (8, 23, "unicorn/no-invalid-property-descriptor"),
            (9, 39, "unicorn/no-invalid-property-descriptor"),
            (11, 8, "unicorn/prefer-object-define-properties"),
            (11, 33, "unicorn/no-invalid-property-descriptor"),
            (11, 59, "unicorn/no-invalid-property-descriptor"),
            (13, 31, "unicorn/no-invalid-property-descriptor"),
            (19, 8, "unicorn/prefer-object-define-properties"),
            (25, 42, "unicorn/prefer-object-define-properties"),
            (27, 31, "unicorn/no-invalid-property-descriptor"),
            (27, 49, "unicorn/no-invalid-property-descriptor"),
            (27, 78, "unicorn/no-invalid-property-descriptor"),
            (28, 31, "unicorn/no-invalid-property-descriptor"),
            (29, 31, "unicorn/no-invalid-property-descriptor"),
            (30, 35, "unicorn/no-invalid-property-descriptor"),
            (31, 15, "unicorn/no-invalid-property-descriptor"),
            (31, 31, "unicorn/prefer-object-define-properties"),
            (32, 18, "unicorn/no-invalid-property-descriptor"),
            (32, 30, "unicorn/no-invalid-property-descriptor"),
        ],
    ),
    (
        "no-unsafe-property-key.ts",
        r"declare const o: any, k: any, ok: boolean;
type Big = 10n; type Neg = -5n; type Huge = 9007199254740993; type Lit = 'a'; type Paren = (object);
type RO = readonly number[]; type Inter = object & {a: 1}; type Mixed = string & {a: 1};
interface Shape { a: number } interface Empty {} interface Child extends Shape {}
class Klass {} enum Color { Red }
type Alias = Shape; type Tuple = [number];
export function f1(a: Big, b: Neg, c: Huge, d: Lit, e: Paren, g: RO, h: Inter, i: Mixed, j: Empty, l: Child, m: Klass, n: Alias, p: Tuple, q: () => void, r: new () => object, s: Array<number>, t: ReadonlyArray<number>, u: bigint, v: -10n, w: unknown, x: Color): void {
  console.log(o[a], o[b], o[c], o[d], o[e], o[g], o[h], o[i], o[j], o[l], o[m], o[n], o[p], o[q], o[r], o[s], o[t], o[u], o[v], o[w], o[x]);
}
const big = 5n; const neg = -1n; const inf = Infinity; const nan = NaN;
console.log(o[big], o[neg], o[inf], o[nan], o[Klass], o[Color], o[Color.Red], o[(k, {})], o[(k, 'a')], o[k satisfies object], o[k as object], o[<object>k], o[k!], o[ok ? 'a' : k], o[ok ? k : {}]);
console.log(o[self.Math], o[window.parseInt], o[globalThis.Symbol], o[Symbol.iterator], o[Number.MAX_SAFE_INTEGER], o[Number.MAX_VALUE]);
const arrow = (): object => ({}); const fn2 = function (): number[] { return []; }; function gen<T>(): T[] { return []; }
console.log(o[arrow()], o[fn2()], o[gen()], o[k()]);
export const obj = { [5n]: 1, 9007199254740993: 2, [k]: 3, get [Klass]() { return 1; }, [Symbol.iterator]: 4 };
export class C2 { [5n] = 1; static [Color.Red] = 2; m() { return 1; } [Klass]() {} accessor [-1n] = 3; }
const { [5n]: d1, [k]: d2 } = o; console.log(d1, d2);
({ [Klass]: o.x } = o);
let declared: object; declared = {}; console.log(o[declared]);
",
        &[
            (7, 53, "unicorn/name-replacements"),
            (7, 80, "unicorn/name-replacements"),
            (7, 90, "unicorn/name-replacements"),
            (8, 17, "unicorn/no-unsafe-property-key"),
            (8, 23, "unicorn/no-unsafe-property-key"),
            (8, 29, "unicorn/no-unsafe-property-key"),
            (8, 41, "unicorn/no-unsafe-property-key"),
            (8, 47, "unicorn/no-unsafe-property-key"),
            (8, 53, "unicorn/no-unsafe-property-key"),
            (8, 71, "unicorn/no-unsafe-property-key"),
            (8, 77, "unicorn/no-unsafe-property-key"),
            (8, 83, "unicorn/no-unsafe-property-key"),
            (8, 89, "unicorn/no-unsafe-property-key"),
            (8, 95, "unicorn/no-unsafe-property-key"),
            (8, 101, "unicorn/no-unsafe-property-key"),
            (8, 107, "unicorn/no-unsafe-property-key"),
            (8, 113, "unicorn/no-unsafe-property-key"),
            (8, 119, "unicorn/no-unsafe-property-key"),
            (8, 125, "unicorn/no-unsafe-property-key"),
            (11, 15, "unicorn/no-unsafe-property-key"),
            (11, 23, "unicorn/no-unsafe-property-key"),
            (11, 31, "unicorn/no-unsafe-property-key"),
            (11, 39, "unicorn/no-unsafe-property-key"),
            (11, 47, "unicorn/no-unsafe-property-key"),
            (11, 57, "unicorn/no-unsafe-property-key"),
            (11, 82, "unicorn/no-unsafe-property-key"),
            (11, 106, "unicorn/no-unsafe-property-key"),
            (11, 129, "unicorn/no-unsafe-property-key"),
            (11, 145, "unicorn/no-unsafe-property-key"),
            (11, 183, "unicorn/no-unsafe-property-key"),
            (12, 15, "unicorn/no-unsafe-property-key"),
            (12, 29, "unicorn/no-unsafe-property-key"),
            (12, 49, "unicorn/no-unsafe-property-key"),
            (12, 119, "unicorn/no-unsafe-property-key"),
            (13, 41, "unicorn/name-replacements"),
            (14, 15, "unicorn/no-unsafe-property-key"),
            (14, 27, "unicorn/no-unsafe-property-key"),
            (15, 14, "unicorn/name-replacements"),
            (15, 23, "unicorn/no-unsafe-property-key"),
            (15, 31, "unicorn/no-unsafe-property-key"),
            (15, 65, "unicorn/no-unsafe-property-key"),
            (16, 20, "unicorn/no-unsafe-property-key"),
            (16, 72, "unicorn/no-unsafe-property-key"),
            (16, 94, "unicorn/no-unsafe-property-key"),
            (17, 9, "unicorn/no-unreadable-object-destructuring"),
            (17, 10, "unicorn/no-unsafe-property-key"),
            (17, 19, "unicorn/no-unreadable-object-destructuring"),
            (18, 4, "unicorn/no-unreadable-object-destructuring"),
            (18, 5, "unicorn/no-unsafe-property-key"),
            (18, 13, "unicorn/no-unreadable-object-destructuring"),
            (19, 52, "unicorn/no-unsafe-property-key"),
        ],
    ),
    (
        "no-unused-builtin-method-return.ts",
        r"declare const list: number[], f: any, g: any, other: Set<number>, s: string, maybe: number[] | undefined;
declare const expect: any;
declare function load(): number[];
const arr = [1, 2]; arr.concat([3]); arr.entries(); arr.every(f); arr.findIndex(f); arr.findLast(f); arr.findLastIndex(f); arr.flat(); arr.flatMap(f); arr.indexOf(1); arr.join(); arr.keys(); arr.lastIndexOf(1); arr.toReversed(); arr.toSorted(); arr.toSpliced(0); arr.with(0, 1);
const set = new Set([1]); set.has(1); set.intersection(other); set.difference(other); set.symmetricDifference(other); set.isSubsetOf(other); set.isSupersetOf(other); set.isDisjointFrom(other);
const ro: ReadonlySet<number> = new Set(); ro.has(1);
let changed = [1]; changed = [2]; changed.map(f);
const alias = arr; alias.map(f);
function p1(x?: number[]): void { x?.map(f); }
function p2(x: number[] | Set<number>): void { x.map(f); }
function p3(x: number[]): void { x.map(f); }
function p4(...xs: number[]): void { xs.map(f); }
function p5({ a }: { a: number[] }): void { a.map(f); }
expect.soft(list).includes(1); expect.poll(f).map(g);
load().map(f);
(list as number[]).map(f); (list as any).map(f); (list satisfies number[]).map(f); list!.map(f); maybe?.map(f);
new Array(3).map(f); new Map().map(f); Number(1).map(f); 'x'.at(0); `x`.at(0); (1 + 2).at(0); (() => 1).at(0); undefined.map(f);
Temporal.Now.instant().add({}); new Temporal.PlainDate(2026, 1, 1).add({ days: 1 }); Temporal.PlainTime.from('10:00').round('hour'); Temporal.PlainMonthDay.from('01-01').add({});
const ins: Temporal.Instant = Temporal.Now.instant(); ins.subtract({ hours: 1 });
const z = Temporal.ZonedDateTime.from('2026-01-01T00:00[UTC]'); z.with({}); z.round('day');
function t1(d: Temporal.Duration): void { d.add({}); }
for (list.map(f); ;) { break; }
(list.map(f), list.map(g));
const kept = (list.map(f), 1); console.log(kept);
class K { m(): void { this.list.map(f); } list = [1]; }
console.log(p1, p2, p3, p4, p5, s, K);
",
        &[
            (4, 7, "unicorn/name-replacements"),
            (4, 25, "unicorn/no-unused-builtin-method-return"),
            (4, 42, "unicorn/no-unused-builtin-method-return"),
            (4, 57, "unicorn/no-unused-builtin-method-return"),
            (4, 71, "unicorn/no-unused-builtin-method-return"),
            (4, 89, "unicorn/no-unused-builtin-method-return"),
            (4, 106, "unicorn/no-unused-builtin-method-return"),
            (4, 128, "unicorn/no-unused-builtin-method-return"),
            (4, 140, "unicorn/no-unused-builtin-method-return"),
            (4, 156, "unicorn/no-unused-builtin-method-return"),
            (4, 172, "unicorn/no-unused-builtin-method-return"),
            (4, 184, "unicorn/no-unused-builtin-method-return"),
            (4, 196, "unicorn/no-unused-builtin-method-return"),
            (4, 216, "unicorn/no-unused-builtin-method-return"),
            (4, 234, "unicorn/no-unused-builtin-method-return"),
            (4, 250, "unicorn/no-unused-builtin-method-return"),
            (4, 268, "unicorn/no-unused-builtin-method-return"),
            (5, 31, "unicorn/no-unused-builtin-method-return"),
            (5, 43, "unicorn/no-unused-builtin-method-return"),
            (5, 68, "unicorn/no-unused-builtin-method-return"),
            (5, 91, "unicorn/no-unused-builtin-method-return"),
            (5, 123, "unicorn/no-unused-builtin-method-return"),
            (5, 146, "unicorn/no-unused-builtin-method-return"),
            (5, 171, "unicorn/no-unused-builtin-method-return"),
            (6, 47, "unicorn/no-unused-builtin-method-return"),
            (8, 26, "unicorn/no-unused-builtin-method-return"),
            (11, 36, "unicorn/no-unused-builtin-method-return"),
            (15, 8, "unicorn/no-unused-builtin-method-return"),
            (16, 20, "unicorn/no-unused-builtin-method-return"),
            (16, 42, "unicorn/no-unused-builtin-method-return"),
            (16, 76, "unicorn/no-unused-builtin-method-return"),
            (16, 90, "unicorn/no-unused-builtin-method-return"),
            (17, 14, "unicorn/no-unreadable-new-expression"),
            (17, 14, "unicorn/no-unused-builtin-method-return"),
            (17, 32, "unicorn/no-unreadable-new-expression"),
            (18, 68, "unicorn/no-unreadable-new-expression"),
            (18, 68, "unicorn/no-unused-builtin-method-return"),
            (18, 119, "unicorn/no-unused-builtin-method-return"),
            (19, 59, "unicorn/no-unused-builtin-method-return"),
            (20, 67, "unicorn/no-unused-builtin-method-return"),
            (20, 79, "unicorn/no-unused-builtin-method-return"),
            (21, 45, "unicorn/no-unused-builtin-method-return"),
            (22, 11, "unicorn/no-unused-builtin-method-return"),
            (23, 7, "unicorn/no-unused-builtin-method-return"),
            (23, 20, "unicorn/no-unused-builtin-method-return"),
            (24, 20, "unicorn/no-unused-builtin-method-return"),
        ],
    ),
    (
        "no-useless-delete-check.ts",
        r"declare const key: string, value: number, k: any, f: () => string;
{ const o: object = {}; if (key in o) { delete (o as any)[key]; } }
{ const o: () => void = () => {}; if ('a' in o) delete (o as any).a; }
{ function decl(): void {} if (key in decl) { delete (decl as any)[key]; } }
{ const o = {} as Record<string, number>; if (key in o) { delete o[key]; } }
{ const o = { a: 1 } as Record<string, number>; if ('a' in o) delete o.a; }
{ const o = [1] as any; if (0 in o) delete o[0]; }
{ const o = {} as any; if (f() in o) delete o[f()]; }
{ const o = {} as any; if ('a' in o) delete o['a']; }
{ const o = {} as any; if ('a' in o) delete o.b; }
{ const o = {} as any; if (`a` in o) delete o[`a`]; }
{ const o = {} as any; const kk = new Date(); if (kk in o) delete o[kk]; }
{ const o = {} as any; const kk = f(); if (kk in o) delete o[kk]; }
{ const o = {} as any; if (k in o) { delete o?.[k]; } }
{ const map = new Map<string, number>(); if (map.has(key) as boolean) { map.delete(key); } }
{ const map = new Map<string, number>(); if (map.has(key)!) map.delete(key) }
{ const map = new Map<string, number>(); if (map.has(key)) { map.delete(key); f(); } }
{ const map = new Map<string, number>(); if (map.has(key)) { } }
{ const map = new Map<string, number>(); if (map?.has(key)) { map.delete(key); } }
{ const map = new Map<string, number>(); if (map.has(...[key])) { map.delete(key); } }
{ const map = new Map<string, number>(); if (value) { map.delete(key); } else if (map.has(key)) { map.delete(key); } }
{ const map = new Map<string, number>(); if (map.has(key)) { /* keep */ map.delete(key); } }
{ const map = new Map<string, number>(); if (map.has(`${key}`)) { map.delete(`${key}`); } }
",
        &[
            (2, 29, "unicorn/no-computed-property-existence-check"),
            (2, 29, "unicorn/no-useless-delete-check"),
            (3, 39, "unicorn/no-useless-delete-check"),
            (4, 12, "unicorn/name-replacements"),
            (4, 32, "unicorn/no-computed-property-existence-check"),
            (5, 47, "unicorn/no-computed-property-existence-check"),
            (5, 47, "unicorn/no-useless-delete-check"),
            (6, 53, "unicorn/no-useless-delete-check"),
            (7, 29, "unicorn/no-useless-delete-check"),
            (8, 28, "unicorn/no-computed-property-existence-check"),
            (9, 28, "unicorn/no-useless-delete-check"),
            (11, 28, "unicorn/no-useless-delete-check"),
            (12, 51, "unicorn/no-computed-property-existence-check"),
            (12, 69, "unicorn/no-unsafe-property-key"),
            (13, 44, "unicorn/no-computed-property-existence-check"),
            (14, 28, "unicorn/no-computed-property-existence-check"),
            (15, 46, "unicorn/no-useless-delete-check"),
            (16, 46, "unicorn/no-useless-delete-check"),
            (22, 46, "unicorn/no-useless-delete-check"),
        ],
    ),
    (
        "objects-more.ts",
        r"declare const object: any, k: string, o: object, getter: () => number, d: PropertyDescriptor;
{ const {[`a`]: v, ...rest} = object; console.log(v, rest); }
{ const {[k as string]: v} = object; console.log(v); }
{ const {a: [b = 1] = []} = object; console.log(b); }
{ let a; ({[k]: a} = object); ({a: [a]} = object); ({a: {b: {c: a}}} = object); console.log(a); }
{ const [{a: {b: [c]}}] = [object]; console.log(c); }
export function fp({a: {b: {c}}}: any, [{d: [e]}]: any): void { console.log(c, e); }
{ const {[Math.PI]: p, [String.raw`a`]: q, ...r} = object; console.log(p, q, r); }
{ const {[k.length]: p, ...r} = object; console.log(p, r); }
export const n1 = new (object.Foo as any)();
export const n2 = new object.a.b.C().d;
export const n3 = new (class {})();
export const n4 = new object.Foo[k]();
export const n5 = new object()?.x;
Object.defineProperty(o, 'p', { get() { return 1; }, value: 1 });
Object.defineProperty(o, 'p', { get: getter, set: 1n, enumerable: 1, configurable: 'yes', writable: null });
Object.defineProperty(o, 'p', { set: [], get: {}, ['value']: 1, [k]: 2 });
Object.defineProperty(o, 'p', { get: `x${k}` });
Object.defineProperties(o, { a: 1, b: { get: 'x' }, ...d });
Object.create(null, { a: { get: getter, writable: true }, b: 'str', c: d });
Reflect['defineProperty'](o, 'p', { foo: 1 });
Object.defineProperty(o, 'p', ...[d]);
Object.defineProperty(o, 'p', { value: 1 } as PropertyDescriptor);
Object.defineProperty(o, 'p', <PropertyDescriptor>{ writable: 1, get: getter });
{ const dd = { writeable: 1 }; Object.defineProperty(o, 'p', dd); Object.defineProperties(o, { x: dd }); }
{ const map = { x: { get: 1 } }; Object.defineProperties(o, map); }
{ const dd = { get: 1 }; const m2 = { y: dd }; Object.create(null, m2); console.log(m2.y); }
Object.defineProperty(o, 'p', { __proto__: null, get: 1 });
Object.defineProperties(o, { __proto__: d, a: { get: 1 } });
export const sym1 = { async [Symbol.iterator]() { return 1; }, [Symbol['dispose']]() {}, async *[Symbol.dispose]() {} };
export class SymC { async [Symbol.dispose](): Promise<void> {} static async [Symbol.asyncDispose](): Promise<void> {} [Symbol.iterator] = async function (): Promise<number> { return 1; }; get [Symbol.dispose]() { return 1; } }
export const sym2 = { [Symbol.dispose]: async (): Promise<void> => {}, [Symbol.asyncDispose]: 1, async [Symbol.dispose]() {} };
export const sym3 = { [Symbol.asyncIterator](): PromiseLike<number> | number { return 1; } };
",
        &[
            (2, 10, "unicorn/no-unreadable-object-destructuring"),
            (3, 10, "unicorn/no-unreadable-object-destructuring"),
            (4, 13, "unicorn/no-unreadable-object-destructuring"),
            (5, 12, "unicorn/no-unreadable-object-destructuring"),
            (5, 36, "unicorn/no-unreadable-object-destructuring"),
            (5, 61, "unicorn/no-unreadable-object-destructuring"),
            (6, 18, "unicorn/no-unreadable-object-destructuring"),
            (7, 28, "unicorn/no-unreadable-object-destructuring"),
            (7, 45, "unicorn/no-unreadable-object-destructuring"),
            (7, 46, "unicorn/name-replacements"),
            (8, 10, "unicorn/no-unreadable-object-destructuring"),
            (8, 24, "unicorn/no-unreadable-object-destructuring"),
            (10, 24, "unicorn/no-unreadable-new-expression"),
            (11, 38, "unicorn/no-unreadable-new-expression"),
            (12, 24, "unicorn/no-unreadable-new-expression"),
            (13, 23, "unicorn/no-unreadable-new-expression"),
            (14, 33, "unicorn/no-unreadable-new-expression"),
            (15, 8, "unicorn/prefer-object-define-properties"),
            (15, 31, "unicorn/no-invalid-property-descriptor"),
            (16, 31, "unicorn/no-invalid-property-descriptor"),
            (16, 51, "unicorn/no-invalid-property-descriptor"),
            (18, 38, "unicorn/no-invalid-property-descriptor"),
            (20, 26, "unicorn/no-invalid-property-descriptor"),
            (20, 62, "unicorn/no-invalid-property-descriptor"),
            (21, 37, "unicorn/no-invalid-property-descriptor"),
            (23, 8, "unicorn/prefer-object-define-properties"),
            (24, 51, "unicorn/no-invalid-property-descriptor"),
            (25, 16, "unicorn/no-invalid-property-descriptor"),
            (26, 27, "unicorn/no-invalid-property-descriptor"),
            (28, 55, "unicorn/no-invalid-property-descriptor"),
            (29, 54, "unicorn/no-invalid-property-descriptor"),
            (30, 30, "unicorn/no-invalid-well-known-symbol-methods"),
            (30, 98, "unicorn/no-invalid-well-known-symbol-methods"),
            (31, 28, "unicorn/no-invalid-well-known-symbol-methods"),
            (31, 120, "unicorn/no-invalid-well-known-symbol-methods"),
            (32, 24, "unicorn/no-invalid-well-known-symbol-methods"),
            (32, 105, "unicorn/no-invalid-well-known-symbol-methods"),
            (33, 24, "unicorn/no-invalid-well-known-symbol-methods"),
        ],
    ),
    (
        "prefer-object-destructuring-defaults.ts",
        r"declare const options: any, x: number, y: any;
{ const {a} = {a: 'x' + 'y', ...options}; console.log(a); }
{ const {a} = {a: !x, ...options}; console.log(a); }
{ const {a} = {a: -x, ...options}; console.log(a); }
{ const {a} = {a: void 0, ...options}; console.log(a); }
{ const {a} = {a: delete y.p, ...options}; console.log(a); }
{ const {a} = {a: x === 1, ...options}; console.log(a); }
{ const {a} = {a: x ** 2, ...options}; console.log(a); }
{ const {a} = {a: x && 1, ...options}; console.log(a); }
{ const {a} = {a: x ? 1 : 2, ...options}; console.log(a); }
{ const {a} = {a: [1, ...[2]], ...options}; console.log(a); }
{ const {a} = {a: [y.p], ...options}; console.log(a); }
{ const {a} = {a: {b: 1}, ...options}; console.log(a); }
{ const {a} = {a: {[x]: 1}, ...options}; console.log(a); }
{ const {a} = {a: {['b']: 1}, ...options}; console.log(a); }
{ const {a} = {a: {...y}, ...options}; console.log(a); }
{ const {a} = {a: (x as number), ...options}; console.log(a); }
{ const {a} = {a: x!, ...options}; console.log(a); }
{ const {a} = {a: /r/u, ...options}; console.log(a); }
{ const {a} = {a: () => 1, ...options}; console.log(a); }
{ const {a} = {a: 1, ...(options as any)}; console.log(a); }
{ const {a} = {a: 1, ...y.p}; console.log(a); }
{ const {a} = {a: 1, get b() { return 1; }, ...options}; console.log(a); }
{ const {a} = {a() { return 1; }, ...options}; console.log(a); }
{ const {1: a} = {1: 'x', ...options}; console.log(a); }
{ const {'a': a} = {'a': 'x', ...options}; console.log(a); }
{ const {a, b} = {a: 1, c: 2, ...options}; console.log(a, b); }
{ const {a = 1} = {a: 2, ...options}; console.log(a); }
{ const {a: {b}} = {a: 1, ...options}; console.log(b); }
{ const [a] = [1]; console.log(a); }
{ const {a} = {...options}; console.log(a); }
{ const {a} = {a: x, ...options}; console.log(a); }
",
        &[
            (2, 9, "unicorn/prefer-object-destructuring-defaults"),
            (5, 9, "unicorn/prefer-object-destructuring-defaults"),
            (7, 9, "unicorn/prefer-object-destructuring-defaults"),
            (8, 9, "unicorn/prefer-object-destructuring-defaults"),
            (9, 9, "unicorn/prefer-object-destructuring-defaults"),
            (10, 9, "unicorn/prefer-object-destructuring-defaults"),
            (11, 9, "unicorn/prefer-object-destructuring-defaults"),
            (13, 9, "unicorn/prefer-object-destructuring-defaults"),
            (15, 9, "unicorn/prefer-object-destructuring-defaults"),
            (16, 9, "unicorn/prefer-object-destructuring-defaults"),
            (17, 9, "unicorn/prefer-object-destructuring-defaults"),
            (18, 9, "unicorn/prefer-object-destructuring-defaults"),
            (19, 9, "unicorn/prefer-object-destructuring-defaults"),
            (20, 9, "unicorn/prefer-object-destructuring-defaults"),
            (21, 9, "unicorn/prefer-object-destructuring-defaults"),
            (25, 9, "unicorn/prefer-object-destructuring-defaults"),
            (26, 9, "unicorn/prefer-object-destructuring-defaults"),
            (32, 9, "unicorn/prefer-object-destructuring-defaults"),
        ],
    ),
    (
        "strings-more.ts",
        r"declare const s: string, n: number, f: any, x: any, o: any, w: string | undefined;
export const r1 = s.replace(/a/gs, '-').replace(/[b]/g, '-').replace(/\t/g, '-').replace(/\x2D/g, '-');
export const r2 = s.replace(/\u0041/g, '').replace(/\cA/g, '').replace(/\0/g, '').replace(/[\b]/g, '');
export const r3 = s.replaceAll('\n', ' ').replaceAll('\r', ' ').replaceAll('\u2028', ' ').replaceAll('\x7F', ' ').replaceAll('\x01', ' ');
export const r4 = s.replace(/a/g, '-') /* c */ .replace(/b/g, '-');
export const r5 = (s + 'x').replace(/a/g, '-').replace(/b/g, '-');
export const r6 = s.replace(/a/g, f).replace(/b/g, f);
export const r7 = s.replace(/\w/g, '').replace(/a/g, '');
export const r8 = s.replace(/[^a]/g, '').replace(/a/g, '');
export const r9 = s.replace(/aa/g, '').replace(/b/g, '');
export const r10 = s.replace(/a/g, '').replace(/[ab]/g, '');
export const r11 = s.replace(/\-/g, '').replace(/\./g, '').replace(/\\/g, '');
export const r12 = s.replace(/^/g, '').replace(/a/g, '');
export const r13 = s?.replaceAll('a', '').replaceAll('b', '');
export const t1 = s.trim().startsWith(n);
export const t2 = s.trim().endsWith(null);
export const t3 = s.trim().endsWith(1n);
export const t4 = (s as string).trim().startsWith('-');
export const t5 = `x`.trim().startsWith('-');
export const t6 = String(n).trim().startsWith('-');
export const t7 = (1).trim().startsWith('-');
export function t8(v: number | undefined): unknown { return (v as any).trim().startsWith('-'); }
export function t9(v: boolean): unknown { return (v as any).trim().startsWith('-'); }
export const t10 = w!.trim().startsWith(['a']);
export const p1 = s.split(':')[0] + s.split(/:/u)['1'] + s.split(':')[-1] + s.split(':')[n];
export const p2 = s.split(':').at(1.5) + s.split(':').at(n) + s.split(':', ...[]).at(0);
export function p3(): void { let a = '', b = ''; [a, b] = s.split(':'); const c = ([a] = s.split(':')); console.log(a, b, c); }
export const p4 = s.split(':')[0]?.trim();
export const d1 = '*'.repeat(10 - s.length) + s;
export const d2 = s + '*'.repeat(n - s.length);
export const d3 = '*'.repeat(10 - o.s.length) + o.s;
export const d4 = (s + '*'.repeat(10)).slice(0, 10);
export const d5 = ('*'.repeat(10) + s).slice(-10);
export const d6 = ('*'.repeat(n) + s).slice(-n);
export const d7 = ('**'.repeat(10) + s).slice(-10);
export const d8 = '\u{1F600}'.repeat(10 - s.length) + s;
export function d9(a: number): string { return '*'.repeat(10 - String(a).length) + a; }
export function d10(a: boolean): string { const b = a; return '*'.repeat(10 - String(b).length) + b; }
export const d11 = `*`.repeat(10 - s.length) + s;
export const d12 = '*'.repeat(10 - s.length) /* c */ + s;
export const d13 = '*'.repeat(10 - x.length) + x;
",
        &[
            (2, 90, "unicorn/prefer-unicode-code-point-escapes"),
            (3, 19, "unicorn/prefer-single-replace"),
            (3, 29, "unicorn/prefer-unicode-code-point-escapes"),
            (3, 52, "unicorn/prefer-unicode-code-point-escapes"),
            (4, 19, "unicorn/prefer-single-replace"),
            (4, 76, "unicorn/prefer-unicode-code-point-escapes"),
            (4, 102, "unicorn/prefer-unicode-code-point-escapes"),
            (4, 126, "unicorn/prefer-unicode-code-point-escapes"),
            (5, 19, "unicorn/prefer-single-replace"),
            (6, 19, "unicorn/prefer-single-replace"),
            (7, 35, "unicorn/no-unsafe-string-replacement"),
            (7, 52, "unicorn/no-unsafe-string-replacement"),
            (12, 20, "unicorn/prefer-single-replace"),
            (16, 21, "unicorn/no-unnecessary-string-trim"),
            (17, 21, "unicorn/no-unnecessary-string-trim"),
            (18, 33, "unicorn/no-unnecessary-string-trim"),
            (19, 23, "unicorn/no-unnecessary-string-trim"),
            (20, 29, "unicorn/no-unnecessary-string-trim"),
            (25, 19, "unicorn/prefer-split-limit"),
            (27, 50, "unicorn/prefer-split-limit"),
            (28, 19, "unicorn/prefer-split-limit"),
            (29, 19, "unicorn/prefer-string-pad-start-end"),
            (30, 19, "unicorn/prefer-string-pad-start-end"),
            (31, 19, "unicorn/prefer-string-pad-start-end"),
            (32, 19, "unicorn/prefer-string-pad-start-end"),
            (33, 19, "unicorn/prefer-string-pad-start-end"),
            (39, 20, "unicorn/prefer-string-pad-start-end"),
            (40, 20, "unicorn/prefer-string-pad-start-end"),
            (41, 20, "unicorn/prefer-string-pad-start-end"),
        ],
    ),
    (
        "strings1.ts",
        r#"declare const name: string, user: { name: string }, x: number, y: number, z: number, value: string, prefix: string, url: string, text: string, count: number, template: string, b: string, list: number[];
declare function gql(s: TemplateStringsArray): string; declare function htmlEscape(s: string): string;
export const g1 = `Hello {name}`;
export const g2 = `Hello $user.name}`;
export const g3 = `${x} {y} and $z}`;
export const g4 = `const x = {foo}`;
export const g5 = `Hello ${name}`;
export const g6 = `Hello ${user.name}`;
export const g7 = gql`query { user { name } }`;
export const g8 = `import {foo} from "bar";`;
export const g9 = `const {foo} = bar;`;
export const g10 = `
	/**
	 * @param {string} id
	 */
`;
export const g11 = `{name + suffix}`;
export const g12 = `{{name}}`;
export const g13 = `\{name}`;
export const g14 = `\u{41}`;
export const g15 = `{1x}`;
export const g16 = `${x}{b`;
export const g17 = `a /* {x} */ {y} ${z} /* $w} */`;
export const g18 = `  export {a, b} from "c"; let {d} = e; var = {f}`;
export const t1 = value.trim().startsWith('-');
export const t2 = value.trim().endsWith('-');
export const t3 = value.trim().startsWith();
export const t4 = value.trim().endsWith(1);
export const t5 = value.trim().startsWith('-', 1);
export const t6 = value.trim().startsWith('foo ');
export const t7 = value.trim().endsWith(' foo');
export const t8 = value.trim().startsWith(prefix);
export const t9 = value?.trim().startsWith('-');
export const t10 = value.trim?.().startsWith('-');
export function t11(v: number[]): unknown { return (v as any).trim().startsWith('-'); }
export function t12(v: string | undefined): unknown { return v!.trim().startsWith('x'); }
template.replace('{url}', htmlEscape(url));
template.replaceAll('{url}', htmlEscape(url));
text.replaceAll('\t', '$&'.repeat(2));
text.replace(/a/gu, `${x}`);
text.replace(/a/gu, value);
template.replace('{url}', () => htmlEscape(url));
template.replace('{url}', 'https://example.com');
template.replace('{url}', `https://example.com`);
text.replaceAll('\t', ' '.repeat(4));
text.replaceAll('\t', ' '.repeat(count));
text.replace('a', String.raw`b`);
text.replace('a', function r() { return 'b'; });
{ const plain = {a: 1}; text.replace('a', plain as any); }
export function t13(l: number[]): unknown { return (l as any).replace('a', b); }
text.replace('a', b as string);
"#,
        &[
            (3, 26, "unicorn/no-incorrect-template-string-interpolation"),
            (4, 26, "unicorn/no-incorrect-template-string-interpolation"),
            (5, 25, "unicorn/no-incorrect-template-string-interpolation"),
            (5, 33, "unicorn/no-incorrect-template-string-interpolation"),
            (6, 30, "unicorn/no-incorrect-template-string-interpolation"),
            (20, 20, "unicorn/prefer-literal-ascii"),
            (23, 33, "unicorn/no-incorrect-template-string-interpolation"),
            (24, 51, "unicorn/no-incorrect-template-string-interpolation"),
            (24, 66, "unicorn/no-incorrect-template-string-interpolation"),
            (25, 25, "unicorn/no-unnecessary-string-trim"),
            (26, 25, "unicorn/no-unnecessary-string-trim"),
            (27, 25, "unicorn/no-unnecessary-string-trim"),
            (28, 25, "unicorn/no-unnecessary-string-trim"),
            (36, 65, "unicorn/no-unnecessary-string-trim"),
            (37, 27, "unicorn/no-unsafe-string-replacement"),
            (38, 30, "unicorn/no-unsafe-string-replacement"),
            (39, 23, "unicorn/no-unsafe-string-replacement"),
            (40, 21, "unicorn/no-unsafe-string-replacement"),
            (41, 21, "unicorn/no-unsafe-string-replacement"),
            (51, 19, "unicorn/no-unsafe-string-replacement"),
        ],
    ),
    (
        "strings2.ts",
        r"declare const string: string, a: string, b: string, s: string, other: number, url: string, path: string, csvLine: string, dynamicSeparator: string, unknownIndex: number, name: string, value: string, width: number, text: string, bar: string, hours: number;
declare const _: { escapeRegExp(v: string): string };
export const e1 = string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
export const e2 = (a + b).replace(/[|\\{}()[\]^$+*?.]/g, '\\$&');
export const e3 = _.escapeRegExp(string);
export const e4 = RegExp.escape(string);
export const e5 = string.replace(/[.*+?^$]/g, '\\$&');
export const e6 = string.replace(/[.*+?^${}()|[\]\\]/gy, '\\$&');
export const e7 = string.replace(/[.*+?^${}()|[\]\\]/g, '$&');
export const r1 = string.replaceAll('a', '-').replaceAll('b', '-').replaceAll('c', '-');
export const r2 = string.replace(/a/g, '_').replaceAll('b', '_');
export const r3 = s.replace(/\./g, '').replace(/[-]/gm, '').replaceAll('^', '').replaceAll(']', '');
export const r4 = s.replace(/\n/g, ' ').replace(/\t/g, ' ');
export const r5 = string.replace('a', '-').replace('b', '-');
export const r6 = string.replaceAll('a', '-').replaceAll('b', '_');
export const r7 = string.replaceAll('a', 'ab').replaceAll('b', 'ab');
export const r8 = string.replace(/a/gi, '-').replaceAll('b', '-');
export const r9 = string.replace(/ab/g, '-').replaceAll('b', '-');
export const r10 = string.replace(/\d/g, '-').replaceAll('b', '-');
export const r11 = s.replace(/\x41/g, '').replace(/[\b]/g, '').replace(/\cJ/g, '');
export const p1 = url.split(':')[0];
export const p2 = csvLine.split(',')[3];
export const p3 = s.split(/,/u).at(1);
export const p4 = s?.split(',')[1 + 1];
export const p5 = string.split('/').at(-1);
export const p6 = string.split(dynamicSeparator)[unknownIndex];
export const p7 = string.split('')[0];
export const p8 = string.split('/', 2)[0];
export const p9 = string.split('/').length;
{ const [, filename] = path.split('/'); console.log(filename); }
{ const [firstPart, ...remainingParts] = string.split('/'); console.log(firstPart, remainingParts); }
{ let x = '', y = ''; [x, y] = s.split(',',); console.log(x, y); }
export const d1 = '*'.repeat(10 - name.length) + name;
export const d2 = value + ' '.repeat(5 - value.length);
export const d3 = '0'.repeat(width - text.length) + text;
export const d4 = ('*'.repeat(10) + bar).slice(-10);
export const d5 = (bar + '*'.repeat(10)).slice(0, 10);
export const d6 = '**'.repeat(10 - name.length) + name;
export const d7 = String(hours).padStart(2, '0');
{ const n = 5; console.log('*'.repeat(10 - String(n).length) + n); }
export function d8(list: number[]): string { return ' '.repeat(3 - list.length) + list.join(); }
export const w1 = '    ';
export const w2 = '\t\t\t';
export const w3 = '\u2003\u2003\u2003';
export const w4 = `   `;
export function w5(): string { return'   '; }
export const w6 = 'aaa';
export const w7 = ' \t ';
export const w8 = '  ';
export const w9 = String.raw`   `;
export const w10 = { '   ': 1 };
type W = '   '; export const w11: W = '   ';
export const w12 = '\u00a0\u00a0\u00a0';
",
        &[
            (3, 14, "unicorn/name-replacements"),
            (3, 19, "unicorn/prefer-regexp-escape"),
            (4, 14, "unicorn/name-replacements"),
            (4, 19, "unicorn/prefer-regexp-escape"),
            (5, 14, "unicorn/name-replacements"),
            (6, 14, "unicorn/name-replacements"),
            (7, 14, "unicorn/name-replacements"),
            (8, 14, "unicorn/name-replacements"),
            (9, 14, "unicorn/name-replacements"),
            (10, 19, "unicorn/prefer-single-replace"),
            (11, 19, "unicorn/prefer-single-replace"),
            (12, 19, "unicorn/prefer-single-replace"),
            (13, 19, "unicorn/prefer-single-replace"),
            (20, 20, "unicorn/prefer-single-replace"),
            (20, 30, "unicorn/prefer-unicode-code-point-escapes"),
            (20, 72, "unicorn/prefer-unicode-code-point-escapes"),
            (21, 19, "unicorn/prefer-split-limit"),
            (22, 19, "unicorn/prefer-split-limit"),
            (23, 19, "unicorn/prefer-split-limit"),
            (24, 19, "unicorn/prefer-split-limit"),
            (30, 9, "unicorn/prefer-split-limit"),
            (32, 23, "unicorn/prefer-split-limit"),
            (33, 19, "unicorn/prefer-string-pad-start-end"),
            (34, 19, "unicorn/prefer-string-pad-start-end"),
            (35, 19, "unicorn/prefer-string-pad-start-end"),
            (36, 19, "unicorn/prefer-string-pad-start-end"),
            (37, 19, "unicorn/prefer-string-pad-start-end"),
            (42, 19, "unicorn/prefer-string-repeat"),
            (43, 19, "unicorn/prefer-string-repeat"),
            (44, 19, "unicorn/prefer-string-repeat"),
            (44, 19, "unicorn/prefer-unicode-code-point-escapes"),
            (45, 19, "unicorn/prefer-string-repeat"),
            (46, 38, "unicorn/prefer-string-repeat"),
            (52, 39, "unicorn/prefer-string-repeat"),
            (53, 20, "unicorn/prefer-string-repeat"),
            (53, 20, "unicorn/prefer-unicode-code-point-escapes"),
        ],
    ),
    (
        "symbols.ts",
        r"declare const value: number, iterator: Iterator<number>, p: Promise<Iterator<number>>, key: symbol;
export class Resource { async [Symbol.dispose](): Promise<void> {} }
export class Resource2 { async [Symbol.asyncDispose](): Promise<void> {} }
export const iterable = { async *[Symbol.iterator]() { yield value; } };
export const iterable2 = { async *[Symbol.asyncIterator]() { yield value; } };
export const asyncIterable = { async [Symbol.asyncIterator]() { return iterator; } };
export const o1 = { *[Symbol.dispose]() { yield 1; } };
export const o2 = { [Symbol.iterator]: (): Promise<Iterator<number>> => p };
export const o3 = { async [Symbol.dispose]() {}, async [Symbol['asyncDispose']]() {} };
export const o4 = { *[Symbol.iterator]() { yield 1; } };
export const o5 = { [Symbol.dispose]() {} };
export const o6 = { get [Symbol.iterator]() { return iterator; } };
export const o7 = { async [key]() {} };
export const o8 = { [Symbol.iterator]: value };
export const o9 = { async [Symbol.dispose]() {} };
export class C { static async [Symbol.dispose](): Promise<void> {} async [Symbol.asyncDispose](): Promise<void> {} [Symbol.iterator] = async () => 1; }
export const o10 = { [Symbol.dispose]: function (): PromiseLike<void> | void { return undefined; } };
",
        &[
            (2, 32, "unicorn/no-invalid-well-known-symbol-methods"),
            (4, 35, "unicorn/no-invalid-well-known-symbol-methods"),
            (6, 39, "unicorn/no-invalid-well-known-symbol-methods"),
            (7, 23, "unicorn/no-invalid-well-known-symbol-methods"),
            (8, 22, "unicorn/no-invalid-well-known-symbol-methods"),
            (9, 28, "unicorn/no-invalid-well-known-symbol-methods"),
            (15, 28, "unicorn/no-invalid-well-known-symbol-methods"),
            (16, 32, "unicorn/no-invalid-well-known-symbol-methods"),
            (16, 117, "unicorn/no-invalid-well-known-symbol-methods"),
            (17, 23, "unicorn/no-invalid-well-known-symbol-methods"),
        ],
    ),
    (
        "unreadable.ts",
        r"declare const Foo: any, foo: any, Bar: any, text: string, object: any, key: string, defaultValue: number, list: any[];
export const b1 = new Foo().getBar();
export const b2 = new Foo().Bar;
export const b3 = new (foo().Bar)();
export const b4 = new foo[Bar]();
export const t = new Date(text).getTime();
export const f0 = new Foo()[0];
export const b5 = new Foo.Bar();
export const b6 = new foo.b.C<number>();
export const b7 = new (Foo as any)();
{ const {[key]: value} = object; console.log(value); }
{ const {foo: [bar]} = object; console.log(bar); }
{ const {foo: {bar: {baz}}} = object; console.log(baz); }
({foo: object.property} = object);
{ const {['a']: value, ...rest} = object; console.log(value, rest); }
export function f({a: [b] = []}: any): void { console.log(b); }
({a: {b: {c: object.c}}} = object);
{ const {foo} = object; console.log(foo); }
{ const {foo: bar = defaultValue} = object; console.log(bar); }
{ const {foo: {bar}} = object; console.log(bar); }
{ const {[key]: value, ...rest} = object; console.log(value, rest); }
{ const [{a: {b}}] = list; console.log(b); }
[object.a] = list;
{ const {[Math.PI]: pi, ...r} = object; console.log(pi, r); }
({a: [object.x]} = object);
({...object.rest} = object);
",
        &[
            (2, 29, "unicorn/no-unreadable-new-expression"),
            (3, 29, "unicorn/no-unreadable-new-expression"),
            (4, 24, "unicorn/no-unreadable-new-expression"),
            (5, 23, "unicorn/no-unreadable-new-expression"),
            (6, 33, "unicorn/no-unreadable-new-expression"),
            (7, 29, "unicorn/no-unreadable-new-expression"),
            (10, 24, "unicorn/no-unreadable-new-expression"),
            (11, 10, "unicorn/no-unreadable-object-destructuring"),
            (12, 15, "unicorn/no-unreadable-object-destructuring"),
            (13, 21, "unicorn/no-unreadable-object-destructuring"),
            (14, 8, "unicorn/no-unreadable-object-destructuring"),
            (15, 10, "unicorn/no-unreadable-object-destructuring"),
            (16, 23, "unicorn/no-unreadable-object-destructuring"),
            (17, 10, "unicorn/no-unreadable-object-destructuring"),
            (17, 14, "unicorn/no-unreadable-object-destructuring"),
            (24, 10, "unicorn/no-unreadable-object-destructuring"),
            (25, 6, "unicorn/no-unreadable-object-destructuring"),
            (26, 6, "unicorn/no-unreadable-object-destructuring"),
        ],
    ),
    (
        "unsafe-key.ts",
        r"declare const object: any, value: number, array: number[], map: any, ok: boolean, key: any, index: number, i: number;
{ const key1 = {}; object[key1] = value; }
{ const key2: never[] = []; console.log({ [key2]: value }); }
array[4n] = value;
console.log({ 9007199254740992: value });
console.log({ [{}]: value });
object[Math] = 1;
object[globalThis.JSON] = 1;
{ function f(): void {} object[f] = 1; }
export function g1(k: object): unknown { return map[k]; }
export function g2(k: number[]): unknown { return map[k]; }
object[Infinity] = 1;
object[ok ? {} : 'a'] = 1;
object[key as bigint] = 1;
console.log({ [-1n]: 1 });
type Point = { x: number }; export function g3(k: Point): unknown { return map[k]; }
{ function make(): number[] { return []; } object[make()] = 1; }
object[String(key)] = value;
console.log({ [key.join('')]: value });
array[Number(4n)] = value;
console.log({ '9007199254740992': value });
console.log(new Map([[{}, value]]));
array[0] = value;
array[index] = value;
array[i + 1] = value;
object[`key`] = 1;
object[Symbol.iterator] = 1;
export function g4(k: string): unknown { return map[k]; }
console.log({ 1.5: 1, 10: 2 });
enum E { A } object[E.A] = 1;
object[ok ? 'a' : 'b'] = 1;
object[NaN] = 1;
object[1e400] = 1;
object[2 ** 53] = 1;
{ const big = 10n; object[big] = 1; }
interface Shape { a: number } export function g5(k: Shape): unknown { return map[k]; }
export function g6(k: readonly string[]): unknown { return map[k]; }
object[new Date()] = 1;
object[(() => 1)] = 1;
{ const { [{}]: x } = object; console.log(x); }
object[parseInt] = 1;
object[window.Reflect] = 1;
object[-0] = 1;
object[0.5] = 1;
",
        &[
            (1, 108, "unicorn/name-replacements"),
            (2, 27, "unicorn/no-unsafe-property-key"),
            (3, 44, "unicorn/no-unsafe-property-key"),
            (4, 7, "unicorn/no-unsafe-property-key"),
            (5, 15, "unicorn/no-unsafe-property-key"),
            (6, 16, "unicorn/no-unsafe-property-key"),
            (7, 8, "unicorn/no-unsafe-property-key"),
            (8, 8, "unicorn/no-unsafe-property-key"),
            (9, 32, "unicorn/no-unsafe-property-key"),
            (10, 53, "unicorn/no-unsafe-property-key"),
            (11, 55, "unicorn/no-unsafe-property-key"),
            (12, 8, "unicorn/no-unsafe-property-key"),
            (13, 8, "unicorn/no-unsafe-property-key"),
            (14, 8, "unicorn/no-unsafe-property-key"),
            (15, 16, "unicorn/no-unsafe-property-key"),
            (16, 80, "unicorn/no-unsafe-property-key"),
            (17, 51, "unicorn/no-unsafe-property-key"),
            (32, 8, "unicorn/no-unsafe-property-key"),
            (33, 8, "unicorn/no-unsafe-property-key"),
            (34, 8, "unicorn/no-unsafe-property-key"),
            (35, 27, "unicorn/no-unsafe-property-key"),
            (36, 82, "unicorn/no-unsafe-property-key"),
            (37, 64, "unicorn/no-unsafe-property-key"),
            (38, 8, "unicorn/no-unsafe-property-key"),
            (39, 9, "unicorn/no-unsafe-property-key"),
            (40, 11, "unicorn/no-unreadable-object-destructuring"),
            (40, 12, "unicorn/no-unsafe-property-key"),
            (41, 8, "unicorn/no-unsafe-property-key"),
            (42, 8, "unicorn/no-unsafe-property-key"),
        ],
    ),
    (
        "unused-props.ts",
        r"declare const x: string, k: string, p: object, f: (v: unknown) => void;
{ const myEnum = { used: 1, unused: 2 }; console.log(myEnum.used); const {used} = myEnum; console.log(used); }
export function foo1(args: {used: number; unused: number}): number { return args.used; }
{ const styles = { success: {}, danger: {} }; f({style: styles.success}); }
{ const o = {a: {b: 1, c: 2}, 0: 1}; console.log(o.a.b); }
export function foo2(args: {used: number}): number { return args.used; }
{ const e1 = { used: 1, usedToo: 2 }; console.log(e1); }
{ const e2 = { used: 1, usedToo: 2 }; console.log(e2.used, e2.usedToo); }
{ const e3 = { used: 1, usedToo: 2 }; console.log(e3[x as 'used']); }
{ const e4 = { used: 1, method() { return 1; } }; console.log(e4.used); }
{ const o1 = {a: 1, b: 2}; console.log(o1); }
export const o2 = {a: 1, b: 2}; console.log(o2.a);
{ const o3 = {a: 1, b: 2}; o3.a = 3; }
{ const o4 = {a: () => 1, b: 2}; o4.a(); }
{ const o5 = {a: 1, b: 2}; const {a, ...rest} = o5; console.log(a, rest); }
{ const o6 = {a: 1, b: 2}; const {[k]: v} = o6; console.log(v); }
{ const o7 = {a: 1, [k]: 2, __proto__: p}; console.log(o7.a); }
{ const o8 = {a: 1, b: 2}; let a = 0, b = 0; ({a, b} = o8); console.log(a, b); }
{ const o9 = {a: 1, b: () => 1}; console.log(o9.a, o9.b); }
{ const o10 = {a: 1, b: 2}; console.log(o10['a'], (o10 as any).c); }
{ const o11: {a: number; b: number} = {a: 1, b: 2}; console.log(o11.a); }
{ const o12: {a: number; b: number} = make(); console.log(o12.a); }
{ const o13 = {a: {x: 1, y: 2}, b: 1}; const z = o13.a; console.log(z, o13.b); }
{ const o14 = {a: {x: 1, y: 2}}; console.log(o14.a.x); let w; w = o14.a; console.log(w); }
{ const o15 = {1: 'a', '2': 'b'}; console.log(o15[1]); }
declare function make(): {a: number; b: number};
",
        &[
            (1, 1, "unicorn/name-replacements"),
            (2, 29, "unicorn/no-unused-properties"),
            (3, 22, "unicorn/name-replacements"),
            (3, 43, "unicorn/no-unused-properties"),
            (4, 33, "unicorn/no-unused-properties"),
            (5, 24, "unicorn/no-unused-properties"),
            (5, 31, "unicorn/no-unused-properties"),
            (6, 22, "unicorn/name-replacements"),
            (7, 9, "unicorn/name-replacements"),
            (8, 9, "unicorn/name-replacements"),
            (9, 9, "unicorn/name-replacements"),
            (10, 9, "unicorn/name-replacements"),
            (16, 35, "unicorn/no-unreadable-object-destructuring"),
            (20, 22, "unicorn/no-unused-properties"),
            (21, 46, "unicorn/no-unused-properties"),
            (22, 26, "unicorn/no-unused-properties"),
            (25, 24, "unicorn/no-unused-properties"),
        ],
    ),
    (
        "unused-return.ts",
        r"declare const array: any[], targetId: number, other: Set<number>, list: number[], a: number, ok: boolean, wrapper: any, Foo: any, x: unknown, expect: any, map: Map<string, number>, f: any, g: any;
declare function transform(e: unknown): unknown; declare function next(): void; declare class Thing { map(f: unknown): void }
array.map(element => transform(element));
array.find(element => element.id === targetId);
{ const set = new Set([1, 2]); set.union(other); }
{ const date = Temporal.PlainDate.from('2026-09-08'); date.add({days: 1}); }
export async function f1(): Promise<void> { await list?.filter(Boolean); }
for (list.map(f); ; list.slice()) { break; }
(list.includes(a), next());
export function f2(items: readonly number[]): void { items.values(); }
export const t1 = array.map(element => transform(element));
void array.map(element => transform(element));
export const c = other.union(other);
array.forEach(f);
array.sort();
other.add(1);
wrapper.items.map(f);
'text'.includes('t');
Foo.map(f);
new Thing().map(f);
String(x).includes('t');
expect(list).includes(1);
map.values();
if (ok) { list.map(f); }
export function f3(l: Set<number>): void { l.has(1); }
export function f4(l: Set<number>): void { (l as any).map(g); }
{ const it = list.values().map(f); it.filter(g); }
list.values();
array.with(0, 1);
array['map'](f);
{ const d: Temporal.Duration = Temporal.Duration.from({hours: 1}); d.round({largestUnit: 'day'}); d.with({}); }
list!.at(0);
(list as number[]).join();
[1, 2].some(Boolean);
Array.from(list).keys();
undefined.map(f);
{ const p = Temporal.Now.plainDateISO(); p.add({days: 1}); }
",
        &[
            (2, 28, "unicorn/name-replacements"),
            (3, 7, "unicorn/no-unused-builtin-method-return"),
            (4, 7, "unicorn/no-unused-builtin-method-return"),
            (5, 36, "unicorn/no-unused-builtin-method-return"),
            (6, 60, "unicorn/no-unused-builtin-method-return"),
            (7, 57, "unicorn/no-unused-builtin-method-return"),
            (8, 11, "unicorn/no-unused-builtin-method-return"),
            (8, 26, "unicorn/no-unused-builtin-method-return"),
            (9, 7, "unicorn/no-unused-builtin-method-return"),
            (10, 60, "unicorn/no-unused-builtin-method-return"),
            (20, 13, "unicorn/no-unreadable-new-expression"),
            (24, 16, "unicorn/no-unused-builtin-method-return"),
            (25, 46, "unicorn/no-unused-builtin-method-return"),
            (28, 6, "unicorn/no-unused-builtin-method-return"),
            (29, 7, "unicorn/no-unused-builtin-method-return"),
            (30, 7, "unicorn/no-unused-builtin-method-return"),
            (31, 70, "unicorn/no-unused-builtin-method-return"),
            (31, 101, "unicorn/no-unused-builtin-method-return"),
            (32, 7, "unicorn/no-unused-builtin-method-return"),
            (33, 20, "unicorn/no-unused-builtin-method-return"),
            (34, 8, "unicorn/no-unused-builtin-method-return"),
            (35, 18, "unicorn/no-unused-builtin-method-return"),
        ],
    ),
];

#[test]
fn gives_the_findings_eslint_gives_on_every_probe() {
    for (name, text, expected) in PROBES {
        let mut found: Vec<(usize, usize, &str)> = RULES
            .iter()
            .filter(|rule| EN_080.contains(&rule.name()))
            .flat_map(|rule| lint(*rule, &format!("scripts/probe/{name}"), text))
            .map(|finding| (finding.line, finding.column, finding.rule))
            .collect();
        found.sort_unstable();
        let mut wanted = expected.to_vec();
        wanted.sort_unstable();
        assert_eq!(found, wanted, "{name}");
    }
}
