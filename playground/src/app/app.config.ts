import { provideBrowserGlobalErrorListeners } from "@angular/core";

import type { ApplicationConfig } from "@angular/core";

/** The app's providers: zoneless, as Angular is without `zone.js` (F50). */
export const appConfig: ApplicationConfig = {
  providers: [provideBrowserGlobalErrorListeners()],
};
