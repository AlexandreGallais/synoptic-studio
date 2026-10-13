import { readGuidedTestSteps } from "./guided-test";
import { mountPlayground } from "./mount-playground";

mountPlayground(document, readGuidedTestSteps(document));
