import mermaid from "mermaid";

/** Code blocks of Mermaid source, as VitePress renders a block of language `mermaid`, not yet taken. */
const SOURCES = "div.language-mermaid:not(.drawing)";

/**
 * SVG of a Mermaid source.
 *
 * @param source - the diagram's Mermaid text
 * @param id - a unique id for the drawing
 * @returns the SVG text, or an empty string when the source has an error: the block then stays
 * as code
 */
async function renderedSource(source: string, id: string): Promise<string> {
  try {
    const { svg } = await mermaid.render(id, source);

    return svg;
  } catch {
    return "";
  }
}

/**
 * Nodes of an SVG drawing written by Mermaid. Its labels hold HTML (`<br>` without a closing
 * slash), which an XML parser refuses: the text is read as HTML.
 *
 * @param svg - SVG text
 * @returns the parsed nodes, ready to insert
 */
export function svgNodes(svg: string): readonly Node[] {
  const parser = new DOMParser();

  return [...parser.parseFromString(svg, "text/html").body.childNodes];
}

/**
 * Draws one Mermaid block in place of its source.
 *
 * @param block - the rendered code block
 * @param index - its number on the page, for a unique diagram id
 * @returns once the diagram has replaced the block
 */
async function drawDiagram(block: Element, index: number): Promise<void> {
  const source = block.querySelector("code")?.textContent ?? "";
  const svg = await renderedSource(source, `diagram-${String(index)}`);

  if (svg === "") {
    return;
  }

  const figure = document.createElement("figure");

  figure.className = "diagram";
  figure.append(...svgNodes(svg));
  block.replaceWith(figure);
}

/**
 * Draws the Mermaid diagrams of the page in the light or dark theme of the site; GitHub draws the
 * same blocks natively.
 *
 * @returns once every diagram is drawn
 * @see docs/conventions/writing.md
 */
export async function drawDiagrams(): Promise<void> {
  const blocks = [...document.querySelectorAll(SOURCES)];

  if (blocks.length === 0) {
    return;
  }

  for (const block of blocks) {
    block.classList.add("drawing");
  }

  const isDark = document.documentElement.classList.contains("dark");

  mermaid.initialize({ startOnLoad: false, theme: isDark ? "dark" : "neutral" });

  await Promise.all(
    blocks.map(async (block, index) => {
      await drawDiagram(block, index);
    }),
  );
}
