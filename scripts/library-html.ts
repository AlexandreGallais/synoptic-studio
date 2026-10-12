// HTML of a documentation page turned into plain Markdown for the local source library
// (docs/tooling/sources.md): headings, paragraphs, lists, code blocks and links kept; scripts,
// styles, navigation and page chrome dropped. A simple converter, enough for documentation pages;
// pages that build their text in the browser are left to WebFetch.

/** What a page states about itself. */
export type PageFacts = {
  /** When it says it was last changed, ISO 8601, or empty. */
  readonly modified: string;
  /** Its `<title>`, or empty. */
  readonly title: string;
};

/** Named entities decoded, beyond the numeric ones. */
const ENTITIES = new Map([
  ["amp", "&"],
  ["apos", "'"],
  ["gt", ">"],
  ["hellip", "…"],
  ["laquo", "«"],
  ["ldquo", "“"],
  ["lsquo", "‘"],
  ["lt", "<"],
  ["mdash", "—"],
  ["nbsp", " "],
  ["ndash", "–"],
  ["quot", '"'],
  ["raquo", "»"],
  ["rdquo", "”"],
  ["rsquo", "’"],
]);

/** Elements dropped with their content: never part of the page's text. */
const DROPPED = [
  "script",
  "style",
  "noscript",
  "svg",
  "nav",
  "header",
  "footer",
  "aside",
  "form",
  "button",
];

/** Patterns of the part of a page that holds its text, tried in order. */
const MAIN_PARTS = [
  /<main\b[^>]*>(?<inner>[\s\S]*?)<\/main>/iu,
  /<article\b[^>]*>(?<inner>[\s\S]*?)<\/article>/iu,
  /<body\b[^>]*>(?<inner>[\s\S]*?)<\/body>/iu,
];

/** Block and inline elements, and the Markdown marks that replace their tags. */
const MARKS: readonly (readonly [RegExp, string])[] = [
  [/<li\b[^>]*>/giu, "\n- "],
  [/<\/?(?:p|div|section|table|tr|ul|ol|dl|dd|dt|blockquote|figure)\b[^>]*>/giu, "\n\n"],
  [/<br\s*\/?>/giu, "\n"],
  [/<\/?(?:code|kbd|samp)\b[^>]*>/giu, "`"],
  [/<\/?(?:strong|b)\b[^>]*>/giu, "**"],
  [/<\/?t[dh]\b[^>]*>/giu, " | "],
];

/** Base of hexadecimal character references. */
const HEX = 16;

/**
 * Character a numeric reference stands for.
 *
 * @param reference - decimal `#38` or hexadecimal `#x26`
 * @returns the character of that code point
 */
function numericCharacter(reference: string): string {
  const isHex = reference[1] === "x" || reference[1] === "X";

  return String.fromCodePoint(
    isHex ? Number.parseInt(reference.slice(2), HEX) : Number(reference.slice(1)),
  );
}

/**
 * Text with its HTML entities decoded.
 *
 * @param text - HTML text
 * @returns the characters the entities stand for; unknown entities kept as written
 */
export function decodeEntities(text: string): string {
  return text.replaceAll(/&(?<name>#x[\da-f]+|#\d+|[a-z]+);/giu, (entity, name: string) =>
    name.startsWith("#") ? numericCharacter(name) : (ENTITIES.get(name.toLowerCase()) ?? entity),
  );
}

/**
 * Text of an HTML fragment, its tags removed.
 *
 * @param html - fragment
 * @returns its text
 */
function withoutTags(html: string): string {
  return html.replaceAll(/<[^>]+>/gu, "");
}

/**
 * The main part of a page: its `<main>` or `<article>` when it has one, else its `<body>`.
 *
 * @param html - whole page
 * @returns the HTML of that part
 */
function mainPart(html: string): string {
  const inner = MAIN_PARTS.map((pattern) => pattern.exec(html)?.groups?.["inner"]).find(
    (part) => part !== undefined,
  );

  return inner ?? html;
}

/**
 * Code blocks as fenced Markdown, their text untouched by the other rules.
 *
 * @param html - HTML part
 * @returns the HTML with each `<pre>` replaced by a fenced block
 */
function fencedCode(html: string): string {
  return html.replaceAll(
    /<pre\b[^>]*>(?<code>[\s\S]*?)<\/pre>/giu,
    (...match: readonly unknown[]) =>
      `\n\n\`\`\`\n${decodeEntities(withoutTags(String(match[1]))).trim()}\n\`\`\`\n\n`,
  );
}

/**
 * Headings and links as Markdown.
 *
 * @param html - HTML part
 * @returns the HTML with `<h1>`–`<h6>` as `#` lines and absolute links as `[label](url)`
 */
function headingsAndLinks(html: string): string {
  return html
    .replaceAll(
      /<h(?<level>[1-6])\b[^>]*>(?<title>[\s\S]*?)<\/h\k<level>>/giu,
      (...match: readonly unknown[]) =>
        `\n\n${"#".repeat(Number(match[1]))} ${withoutTags(String(match[2])).trim()}\n\n`,
    )
    .replaceAll(
      /<a\b[^>]*href="(?<href>[^"]*)"[^>]*>(?<label>[\s\S]*?)<\/a>/giu,
      (...match: readonly unknown[]) => {
        const href = String(match[1]);

        return href.startsWith("http") ? `[${String(match[2])}](${href})` : String(match[2]);
      },
    );
}

/**
 * A documentation page as Markdown.
 *
 * @param html - the page's HTML
 * @returns its main text with headings, lists, code blocks and links; blank lines collapsed
 */
export function htmlToMarkdown(html: string): string {
  const kept = DROPPED.reduce(
    (part, tag) => part.replaceAll(new RegExp(String.raw`<${tag}\b[\s\S]*?</${tag}>`, "giu"), ""),
    mainPart(html).replaceAll(/<!--[\s\S]*?-->/gu, ""),
  );
  const marked = MARKS.reduce(
    (part, [pattern, mark]) => part.replaceAll(pattern, () => mark),
    headingsAndLinks(fencedCode(kept)),
  );

  return decodeEntities(withoutTags(marked))
    .split("\n")
    .map((line) => line.replaceAll(/[ \t]+/gu, " ").trimEnd())
    .join("\n")
    .replaceAll(/\n{3,}/gu, "\n\n")
    .trim();
}

/**
 * First value a page gives for a fact, among several patterns.
 *
 * @param html - the page's HTML
 * @param patterns - patterns with a `value` group, tried in order
 * @returns the value, entities decoded and trimmed, or empty
 */
function firstValue(html: string, patterns: readonly RegExp[]): string {
  const value = patterns
    .map((pattern) => pattern.exec(html)?.groups?.["value"])
    .find((found) => found !== undefined);

  return decodeEntities(value ?? "").trim();
}

/**
 * Title and last change a page states about itself.
 *
 * @param html - whole page as served
 * @returns its `<title>`, and its modification date from its meta tags or its JSON-LD
 */
export function pageFacts(html: string): PageFacts {
  return {
    modified: firstValue(html, [
      /property="article:modified_time"\s+content="(?<value>[^"]+)"/iu,
      /"dateModified"\s*:\s*"(?<value>[^"]+)"/iu,
    ]),
    title: firstValue(html, [/<title\b[^>]*>(?<value>[\s\S]*?)<\/title>/iu]),
  };
}
