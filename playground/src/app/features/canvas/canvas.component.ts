import { Component, DOCUMENT, inject } from "@angular/core";

import { mountPlayground } from "../../shared/utils/mount";

import type { AfterViewInit } from "@angular/core";

/**
 * The working area: the drawing and the layer of helps above it. Once its view is ready, it
 * calls the page's plain-TypeScript mount, whose pointer listeners and drawing stay outside
 * Angular's reactivity (EN-160).
 */
@Component({
  selector: "app-canvas",
  styleUrl: "./canvas.component.scss",
  templateUrl: "./canvas.component.html",
})
export class CanvasComponent implements AfterViewInit {
  /** The page's document, which the mount listens to and draws into. */
  readonly #document = inject(DOCUMENT);

  /**
   * Mounts the page. Every feature's markup is in the document by then: Angular creates the
   * whole tree of views before it calls a view's hooks.
   */
  ngAfterViewInit(): void {
    mountPlayground(this.#document);
  }
}
