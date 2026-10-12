import { mountPlayground } from "./mount-playground";
import { readGuidedTestSteps } from "./read-guided-test-steps";

mountPlayground(document, readGuidedTestSteps(document));
