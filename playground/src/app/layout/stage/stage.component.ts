import { Component } from "@angular/core";

import { CanvasComponent } from "../../features/canvas/canvas.component";
import { ToolbarComponent } from "../../features/toolbar/toolbar.component";

/** The stage: the canvas, with the toolbar floating over it. */
@Component({
  selector: "app-stage",
  imports: [CanvasComponent, ToolbarComponent],
  templateUrl: "./stage.component.html",
  styleUrl: "./stage.component.scss",
})
export class StageComponent {}
