import { mountChecklist } from "./checklist";
import { readGuidedTestSteps } from "./guided-test";
import { mountPlayground } from "./mount";

mountPlayground(document, readGuidedTestSteps(document));
mountChecklist(document);
