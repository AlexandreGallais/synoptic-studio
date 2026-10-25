import { loadPage } from "./load-page";

/**
 * Opens the playground at an address exactly as the browser does: the app loaded, then its first
 * change detection, after which the app mounts the page.
 *
 * @kind procedure
 * @param search - query of the address, or empty
 */
export function openPage(search: string): void {
  loadPage(search).changeDetectorRef.detectChanges();
}
