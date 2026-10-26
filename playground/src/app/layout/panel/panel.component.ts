import { Component } from "@angular/core";

import { PropertiesComponent } from "../../features/properties/properties.component";

/** The right panel: the properties of what is edited. */
@Component({
  selector: "app-panel",
  imports: [PropertiesComponent],
  templateUrl: "./panel.component.html",
  styleUrl: "./panel.component.scss",
})
export class PanelComponent {}
