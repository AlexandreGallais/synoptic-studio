import type { Point } from "editor";

/** A drag that pans the view (SP-030's answer 2): `Space` held with the main button, or the middle button. */
export type PanDrag = {
  /** The pointer's last place in the window, in CSS pixels. */
  readonly client: Point;
  /** Whether it is released: the click that follows it is the pan's, never a pick nor a shape placed. */
  readonly isReleased: boolean;
};
