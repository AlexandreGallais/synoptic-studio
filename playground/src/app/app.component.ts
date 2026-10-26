import { Component, ViewEncapsulation } from "@angular/core";

import { OutputComponent } from "./layout/output/output.component";
import { PanelComponent } from "./layout/panel/panel.component";
import { StageComponent } from "./layout/stage/stage.component";

/** The app's root: the stage, the right panel and the debug panel side by side (EN-160). */
@Component({
  // The page's style crosses the components, as `index.html`'s did, until the zones follow Figma.
  encapsulation: ViewEncapsulation.None,
  imports: [StageComponent, PanelComponent, OutputComponent],
  selector: "app-root",
  styleUrl: "./app.component.scss",
  templateUrl: "./app.component.html",
})
export class AppComponent {}
