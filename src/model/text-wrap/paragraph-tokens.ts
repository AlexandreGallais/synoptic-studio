/**
 * A token: a run of characters other than the space U+0020 and the spaces after it; the first one
 * also takes the spaces that open the paragraph.
 */
const TOKEN = /^ *[^ ]* *|[^ ]+ */gu;

/**
 * The tokens of a paragraph, between which its lines may wrap: a break is allowed after the last
 * space of each run of spaces U+0020 (LB18), never at a no-break space U+00A0 (LB12), which stays
 * inside its token; the spaces that open the paragraph join its first token, so that no line is
 * made of them alone (`DERIV-text-box`, step 1).
 *
 * @kind domain
 * @param paragraph - a paragraph of a text, between two line breaks typed
 * @returns its tokens, in order, which joined give it back; none for an empty paragraph
 * @see DERIV-text-box
 */
export function paragraphTokens(paragraph: string): readonly string[] {
  return Array.from(paragraph.matchAll(TOKEN), ([token]) => token).filter((token) => token !== "");
}
