import { cyclicItem } from "../cyclic-item";

import { cornerSetbacks } from "./corner-setbacks";
import { edgeFactors } from "./edge-factors";
import { edgeLengths } from "./edge-lengths";
import { vertexFactors } from "./vertex-factors";

import type { Corner } from "../corner";

/**
 * Effective radius of every corner from its turning angle, already computed: requested radii
 * reduced locally and proportionally so that the arcs of an edge never overlap (ADR-0007).
 *
 * Formula: r′ᵢ = gᵢ · rᵢ, with gᵢ = min(fᵢ₋₁, fᵢ) and fᵢ = edgeFactor(Lᵢ, sᵢ + sᵢ₊₁).
 *
 * @kind geometry
 * @param corners - vertices with their requested radii, in drawing order
 * @param angles - turning angle `τᵢ` of each corner, in drawing order
 * @returns one effective radius per corner, 0 <= r′ <= r
 * @see DERIV-local-radius-clamp
 */
export function clampedRadii(
  corners: readonly Corner[],
  angles: readonly number[],
): readonly number[] {
  const lengths = edgeLengths(corners.map((corner) => corner.point));
  const factors = vertexFactors(edgeFactors(lengths, cornerSetbacks(corners, angles)));

  return corners.map((corner, index) => corner.radius * cyclicItem(factors, index, 1));
}
