import { Component, DOCUMENT, afterNextRender, inject } from "@angular/core";

import { mountPlayground } from "../../shared/utils/mount";

/**
 * The working area: the drawing and the layer of helps above it. Once the app has rendered, it
 * calls the page's plain-TypeScript mount, whose pointer listeners and drawing stay outside
 * Angular's reactivity (EN-160).
 */
@Component({
  selector: "app-canvas",
  styleUrl: "./canvas.component.scss",
  templateUrl: "./canvas.component.html",
})
export class CanvasComponent {
  /** The page's document, which the mount listens to and draws into. */
  readonly #document = inject(DOCUMENT);

  /** Mounts the page after the app's first render, when every feature holds its markup. */
  constructor() {
    afterNextRender(() => {
      mountPlayground(this.#document);
    });
  }
}
