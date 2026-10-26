import { Component } from "@angular/core";

import { OutputComponent } from "./layout/output/output.component";
import { PanelComponent } from "./layout/panel/panel.component";
import { StageComponent } from "./layout/stage/stage.component";

/**
 * The app's root: the stage, the right panel and the debug panel side by side (EN-160). The page's
 * style is global, the app's `styles` entry, `shared/styles/page.scss`.
 */
@Component({
  selector: "app-root",
  imports: [StageComponent, PanelComponent, OutputComponent],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent {}
