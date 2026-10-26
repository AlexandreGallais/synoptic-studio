import { EnvironmentInjector, createComponent } from "@angular/core";
import { TestBed } from "@angular/core/testing";

// lint-disable-next-line local/canonical-import-path -- the rule names the first folder under the common ancestor, playground/src, which has no barrel in the Angular app
import { AppComponent } from "../src/app/app.component";
// lint-disable-next-line local/canonical-import-path -- the rule names the first folder under the common ancestor, playground/src, which has no barrel in the Angular app
import { appConfig } from "../src/app/app.config";

import { createResizeObserver } from "./create-resize-observer";
import { pageStyle } from "./page-style-fixtures";

import type { ComponentRef } from "@angular/core";

/**
 * Loads the playground's Angular app at an address, as the browser serves it, without mounting
 * the page: the app's root element alone in the body, the app created into it with its
 * providers, its features holding the page's markup and its style in the head, the page loaded
 * before replaced; its animation frames run at once, and its `ResizeObserver` is a stand-in the
 * test tells of a size change (`runResizeObservers`). The app's first change detection, which
 * mounts the page, is the test's (`openPage`).
 *
 * The app is no `TestBed` fixture and its module is kept at the cleaning `TestBed` runs around
 * each test, so that a page opened before all the tests of a file lasts through them.
 *
 * @kind procedure
 * @param search - query of the address, or empty
 * @returns the app, whose first change detection mounts the page
 */
export function loadPage(search: string): ComponentRef<AppComponent> {
  globalThis.history.replaceState(undefined, "", `/${search}`);
  TestBed.resetTestingModule();

  TestBed.configureTestingModule({
    providers: [...appConfig.providers],
    teardown: { destroyAfterEach: false },
  });

  // The page loaded before replaced, its style with it, so that each id is the new page's alone;
  // the page's global style first, as the app's `styles` entry is in the browser.
  const root = document.createElement("app-root");
  const style = document.createElement("style");

  style.textContent = pageStyle();
  document.head.replaceChildren(style);
  document.body.replaceChildren(root);

  // A test reads the page right after acting on it: its animation frames run at once, so that
  // a redraw asked for the next frame is drawn before the test reads it.
  Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
    configurable: true,
    value: (callback: FrameRequestCallback): number => {
      callback(0);

      return 0;
    },
  });

  // happy-dom's `ResizeObserver` observes nothing: a stand-in the test calls back.
  Object.defineProperty(document.defaultView ?? {}, "ResizeObserver", {
    configurable: true,
    value: createResizeObserver,
  });

  return createComponent(AppComponent, {
    environmentInjector: TestBed.inject(EnvironmentInjector),
    hostElement: root,
  });
}
