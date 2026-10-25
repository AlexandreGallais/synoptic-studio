import { Component } from "@angular/core";

import { CanvasComponent } from "../../features/canvas/canvas.component";
import { ToolbarComponent } from "../../features/toolbar/toolbar.component";

/** The stage: the canvas, with the toolbar floating over it. */
@Component({
  imports: [CanvasComponent, ToolbarComponent],
  selector: "app-stage",
  styleUrl: "./stage.component.scss",
  templateUrl: "./stage.component.html",
})
export class StageComponent {}
