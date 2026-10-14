import { describe, expect, it } from "vitest";

import { decodeEntities, htmlToMarkdown, pageFacts } from "./library-html";

describe("decodeEntities", () => {
  it("decodes named, decimal and hexadecimal references, and keeps unknown ones", () => {
    expect(decodeEntities("a &amp; b &#38; c &#x26; &lt;p&gt; &mdash; &unknown;")).toBe(
      "a & b & c & <p> — &unknown;",
    );
  });
});

describe("htmlToMarkdown", () => {
  it("removes nested tags and comments, keeping a < that opens no tag", () => {
    expect(htmlToMarkdown("<body><p><<b>i>x<!<!-- a -->-- b --></p></body>")).toBe("x");
  });

  it("keeps the main part as Markdown and drops the page chrome", () => {
    const html = [
      "<html><head><title>T</title><script>x()</script></head><body>",
      "<nav>Menu</nav><main><h1>Title</h1><p>One <strong>bold</strong> <code>x</code>.</p>",
      '<ul><li>first</li><li><a href="https://a.org">second</a></li><li><a href="/local">third</a></li></ul>',
      "<pre><code>let a = 1 &lt; 2;\n</code></pre><!-- note --><footer>Foot</footer></main></body></html>",
    ].join("");

    expect(htmlToMarkdown(html)).toBe(
      [
        "# Title",
        "",
        "One **bold** `x`.",
        "",
        "- first",
        "- [second](https://a.org)",
        "- third",
        "",
        "```",
        "let a = 1 < 2;",
        "```",
      ].join("\n"),
    );
  });

  it("falls back on the article, then on the body", () => {
    expect(htmlToMarkdown("<body><article><p>A</p></article><p>B</p></body>")).toBe("A");
    expect(htmlToMarkdown("<body><p>B</p></body>")).toBe("B");
  });
});

describe("pageFacts", () => {
  it("reads the title and the modification date from meta tags or JSON-LD", () => {
    expect(
      pageFacts(
        '<title>Doc &amp; more</title><meta property="article:modified_time" content="2026-09-01T10:00:00Z">',
      ),
    ).toEqual({ modified: "2026-09-01T10:00:00Z", title: "Doc & more" });

    expect(pageFacts('<script>{"dateModified": "2026-08-02"}</script>').modified).toBe(
      "2026-08-02",
    );

    expect(pageFacts("<p>nothing</p>")).toEqual({ modified: "", title: "" });
  });
});
