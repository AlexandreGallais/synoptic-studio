import { Component } from "@angular/core";

import { PipelineComponent } from "../../features/debug/pipeline/pipeline.component";

/** The debug panel: the pipeline's output, shown or hidden by the toolbar. */
@Component({
  selector: "app-output",
  imports: [PipelineComponent],
  templateUrl: "./output.component.html",
  styleUrl: "./output.component.scss",
})
export class OutputComponent {}
