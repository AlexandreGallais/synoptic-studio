import { Component } from "@angular/core";

import { PropertiesComponent } from "../../features/properties/properties.component";

/** The right panel: the properties of what is edited. */
@Component({
  imports: [PropertiesComponent],
  selector: "app-panel",
  styleUrl: "./panel.component.scss",
  templateUrl: "./panel.component.html",
})
export class PanelComponent {}
