import { readFontSession, writeFontOptions, writeFontSession } from "../fonts";
import { runTask, selectElement, writeText } from "../page";

import { readFamilyFonts } from "./read-family-fonts";
import { readInstalledFamily } from "./read-installed-family";
import { readInstalledFontsAhead } from "./read-installed-fonts-ahead";

import type { InstalledFont } from "../fonts";

/** A window that lists the installed fonts: Chrome and Edge from version 103 (`REF-CHROME-LOCAL-FONTS`). */
type LocalFontAccess = {
  /** Asks the permission `local-fonts` once, then lists the installed faces. */
  readonly queryLocalFonts: () => Promise<readonly InstalledFont[]>;
};

/** Said where the browser cannot list the installed fonts. */
const UNSUPPORTED =
  "This browser cannot list the installed fonts (Chrome and Edge can): use Import a font with the font's file.";

/**
 * Reads the window's access to the installed fonts, detected as Chrome advises
 * (`'queryLocalFonts' in window`, `REF-CHROME-LOCAL-FONTS`).
 *
 * @kind procedure
 * @param view - the page's window
 * @returns the window in Chrome and Edge; undefined in Firefox and Safari
 * @see REF-CHROME-LOCAL-FONTS
 */
function readLocalFontAccess(view: object): LocalFontAccess | undefined {
  return "queryLocalFonts" in view && typeof view.queryLocalFonts === "function"
    ? // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- detected as Chrome advises; the DOM types do not hold this draft API, which answers a list of FontData
      (view as LocalFontAccess)
    : undefined;
}

/**
 * Shows the installed fonts, after the browser's permission, in the font list, after the
 * families kept, then reads their files in the background (`readInstalledFontsAhead`, US-053).
 *
 * @kind procedure
 * @param document - page document
 * @param access - the window that lists them
 * @param update - redraws the playground
 * @returns once they are listed
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
async function showInstalledFonts(
  document: Document,
  access: LocalFontAccess,
  update: () => void,
): Promise<void> {
  const installed = await access.queryLocalFonts();
  const families = new Set(installed.map((font) => font.family));

  writeFontSession(document, { ...readFontSession(document), installed });
  writeFontOptions(document, "");

  writeText(
    document,
    "font-message",
    `Installed families listed: ${String(families.size)}. Choose one in the font list.`,
  );

  update();

  runTask(
    document,
    async () => {
      await readInstalledFontsAhead(document, update);
    },
    (error) => {
      writeText(document, "installed-fonts-progress", String(error));
    },
  );
}

/**
 * Reads the error of a list refused: the permission refused (`NotAllowedError`), or another
 * reason.
 *
 * @kind procedure
 * @param error - the error the browser gave
 * @returns the line said
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readListError(error: unknown): string {
  return error instanceof DOMException && error.name === "NotAllowedError"
    ? "The installed fonts are not listed: the browser's permission was refused."
    : `The installed fonts could not be listed: ${String(error)}`;
}

/**
 * Reads the files of an installed family chosen in the font list (`readInstalledFamily`), says
 * what was read, then redraws; meanwhile the list shows it chosen, "Reading …" is said, and the
 * text keeps the family chosen before it (`FontSession.measured`), never Inter by default, or is
 * measured at once with a family read ahead, its metrics kept (US-053). A family whose files are
 * held, imported or read when chosen, is not read again.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function listenToInstalledChoice(document: Document, update: () => void): void {
  const font = selectElement(document, "text-font");

  font.addEventListener("change", () => {
    const family = font instanceof HTMLSelectElement ? font.value : "";
    const session = readFontSession(document);

    if (readFamilyFonts(session, family).length === 0) {
      return;
    }

    writeText(document, "font-message", `Reading ${family}…`);

    runTask(
      document,
      async () => {
        writeText(document, "font-message", await readInstalledFamily(document, family, update));
        update();
      },
      update,
    );
  });
}

/**
 * Listens to **Installed fonts** and to the font list: a click asks the browser to list the
 * installed fonts — a click being needed for the permission's prompt (`REF-WICG-LOCAL-FONT-ACCESS`)
 * — and says so where the browser cannot (Firefox, Safari) or the permission is refused; an
 * installed family chosen has its files read.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see REF-CHROME-LOCAL-FONTS
 */
export function listenToInstalledFonts(document: Document, update: () => void): void {
  selectElement(document, "installed-fonts").addEventListener("click", () => {
    const access = readLocalFontAccess(document.defaultView ?? {});

    if (access === undefined) {
      writeText(document, "font-message", UNSUPPORTED);

      return;
    }

    runTask(
      document,
      async () => {
        await showInstalledFonts(document, access, update);
      },
      (error) => {
        writeText(document, "font-message", readListError(error));
      },
    );
  });

  listenToInstalledChoice(document, update);
}
