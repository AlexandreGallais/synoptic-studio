import { Component } from "@angular/core";

import { PipelineComponent } from "../../features/debug/pipeline/pipeline.component";

/** The debug panel: the pipeline's output, shown or hidden by the toolbar. */
@Component({
  imports: [PipelineComponent],
  selector: "app-output",
  styleUrl: "./output.component.scss",
  templateUrl: "./output.component.html",
})
export class OutputComponent {}
