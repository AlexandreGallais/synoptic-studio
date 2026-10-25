import { selectElement } from "./select-element";

/** An option of a choice: the value it holds and the label it shows. */
type SelectOption = {
  /** Its `value`. */
  readonly value: string;
  /** What it shows. */
  readonly label: string;
};

/** A choice of the page, its options and the value chosen. */
type SelectChoice = {
  /** The choice's `id`, without `#`. */
  readonly id: string;
  /** Its options, in order. */
  readonly options: readonly SelectOption[];
  /** The value chosen, one of its options'. */
  readonly value: string;
};

/**
 * Writes the options of a choice of the page and chooses one: the options are written again only
 * when their values change, so that a list kept as it is keeps its state in the page.
 *
 * @kind procedure
 * @param document - page document
 * @param choice - the choice's id, its options and the value chosen
 */
export function writeSelectOptions(document: Document, choice: SelectChoice): void {
  const select = selectElement(document, choice.id);

  if (!(select instanceof HTMLSelectElement)) {
    return;
  }

  const shown = [...select.options].map((option) => option.value);

  if (shown.join("\n") !== choice.options.map((option) => option.value).join("\n")) {
    select.replaceChildren(
      ...choice.options.map((option) =>
        Object.assign(document.createElement("option"), {
          textContent: option.label,
          value: option.value,
        }),
      ),
    );
  }

  select.value = choice.value;
}
