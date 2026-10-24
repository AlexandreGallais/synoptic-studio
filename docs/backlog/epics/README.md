# Epics

Ordered by the Product Owner (2026-10-08; E13 added before E03, confirmed 2026-10-10; E14 and E15 added after E01 on 2026-10-12; E17 added before E02 on 2026-10-16, at RET-003; E18 and E19 cut from E02 and placed after it on 2026-10-19, SP-025). Drafts written by Claude Code from the Product Owner's description; none is `ready` until the Product Owner says so. Every epic of this table makes the free V1 (ADR-0030, Q22).

**The library first, the interface last** (Product Owner, 2026-10-19): the epics that build the library and its playground come first, and may offer more options than a designer will need, so that they move fast; the design of the editors' interface comes after them all, in E20, where each button and option is questioned — whether it matters, whether it is said twice in two places, whether it is clear, whether it helps or slows the designer. Within that rule the agent orders the epics.

| Order | ID | Epic | Status |
| --- | --- | --- | --- |
| 1 | [E01](./E01-draw-shapes.md) | Draw symbol shapes from numbers | done |
| 2 | [E14](./E14-draw-curves.md) | Draw curves with handles, ellipses and round shapes | done |
| 3 | [E15](./E15-place-text.md) | Place text in symbols | done |
| 4 | [E17](./E17-fast-tooling-with-oxc-and-rust.md) | Fast tooling with Oxc and Rust | done |
| 5 | [E02](./E02-edit-shapes.md) | Select and move shapes on one drawing | done |
| 6 | [E18](./E18-transform-shapes-by-hand.md) | Transform shapes and points by hand | draft |
| 7 | [E19](./E19-snap-align-and-distribute.md) | Snap, align and distribute shapes | draft |
| 8 | [E13](./E13-arrange-shapes-in-a-tree.md) | Arrange shapes in a tree and lay them out | draft |
| 9 | [E03](./E03-combine-shapes.md) | Combine shapes without losing their numbers | draft |
| 10 | [E04](./E04-outline-shapes.md) | Give shapes colors and exact strokes | draft |
| 11 | [E05](./E05-build-animatable-symbols.md) | Build animatable symbols | draft |
| 12 | [E06](./E06-configure-business-types.md) | Configure business types in cascade | draft |
| 13 | [E07](./E07-publish-business-libraries.md) | Publish business symbol libraries | draft |
| 14 | [E08](./E08-assemble-views.md) | Assemble synoptic views | draft |
| 15 | [E09](./E09-route-pipes.md) | Connect symbols with orthogonal pipes | draft |
| 16 | [E10](./E10-draw-static-drawings.md) | Draw and reuse static drawings | draft |
| 17 | [E11](./E11-store-and-share.md) | Store and share as files in a working folder | draft |
| 18 | [E20](./E20-make-the-editor-easy-to-use.md) | Make the editor easy to use | draft |
| 19 | [E12](./E12-release-a-trusted-product.md) | Release a product people can trust | draft |

## Ideas for later

Draft epics recorded from the Product Owner's ideas, outside the run order until the Product Owner places them; their edition, free V1 or pro, is decided then (ADR-0030).

| ID | Epic | Given | Status |
| --- | --- | --- | --- |
| [E16](./E16-check-text-contrast.md) | Check the contrast of texts on their background | 2026-10-15 | draft |

## Next (REV-008, 2026-10-24)

Before the next epic starts, the epics not done are regrouped into small epics of one subject each (ADR-0100, decision 1), the Product Owner allowing them all to be dropped and written again: a spike from `develop` proposes the new list, each epic with its features and their sizes, and the Product Owner orders it. The helps left in the drawing's SVG (the box frames and the points' handles) go to one of them.
