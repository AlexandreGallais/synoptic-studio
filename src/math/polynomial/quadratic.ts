/** Coefficients of a quadratic `a t² + b t + c`. */
export type Quadratic = {
  /** Coefficient of t². */
  readonly a: number;
  /** Coefficient of t. */
  readonly b: number;
  /** Constant term. */
  readonly c: number;
};
