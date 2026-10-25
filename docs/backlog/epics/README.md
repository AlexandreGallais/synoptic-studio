# Epics

Ordered by the Product Owner (2026-10-08; E13 added before E03, confirmed 2026-10-10; E14 and E15 added after E01 on 2026-10-12; E17 added before E02 on 2026-10-16, at RET-003; E18 and E19 cut from E02 and placed after it on 2026-10-19, SP-025); the epics not done regrouped into E21 to E48, small epics of one subject each, ordered on 2026-10-25 (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md)); E49 and E50, the agent's knowledge and setup, added before E21 the same day, since they speed up every epic after them. Drafts written by Claude Code from the Product Owner's description; none is `ready` until the Product Owner says so. Every epic of this table makes the free V1 (ADR-0030, Q22).

**The library first, the interface last** (Product Owner, 2026-10-19): the epics that build the library and its playground come first, and may offer more options than a designer will need, so that they move fast; the design of the editors' interface comes after them all, in E45, where each button and option is questioned — whether it matters, whether it is said twice in two places, whether it is clear, whether it helps or slows the designer. The desktop application comes just before the release's epics (Product Owner, 2026-10-25). Within that rule the agent orders the epics.

| Order | ID | Epic | Status |
| --- | --- | --- | --- |
| 1 | [E01](./E01-draw-shapes.md) | Draw symbol shapes from numbers | done |
| 2 | [E14](./E14-draw-curves.md) | Draw curves with handles, ellipses and round shapes | done |
| 3 | [E15](./E15-place-text.md) | Place text in symbols | done |
| 4 | [E17](./E17-fast-tooling-with-oxc-and-rust.md) | Fast tooling with Oxc and Rust | done |
| 5 | [E02](./E02-edit-shapes.md) | Select and move shapes on one drawing | done |
| 6 | [E49](./E49-keep-the-agents-knowledge-in-a-database.md) | Keep the agent's knowledge in a database, the docs light | in-progress |
| 7 | [E50](./E50-share-the-agents-setup-with-every-agent.md) | Share the agent's setup with every coding agent | draft |
| 8 | [E21](./E21-resize-shapes-by-their-box.md) | Resize shapes by their box | draft |
| 9 | [E22](./E22-turn-shapes-about-a-chosen-center.md) | Turn shapes about a chosen center | draft |
| 10 | [E23](./E23-drag-points-and-handles.md) | Drag points and handles | draft |
| 11 | [E24](./E24-compute-in-number-fields.md) | Compute in number fields | draft |
| 12 | [E25](./E25-keep-the-history-fast.md) | Keep the history fast | draft |
| 13 | [E26](./E26-snap-align-and-distribute-shapes.md) | Snap, align and distribute shapes | draft |
| 14 | [E27](./E27-group-shapes-and-order-their-stacking.md) | Group shapes and order their stacking | draft |
| 15 | [E28](./E28-lay-out-shapes-in-boxes.md) | Lay out shapes in boxes | draft |
| 16 | [E29](./E29-combine-shapes-by-boolean-operations.md) | Combine shapes by boolean operations | draft |
| 17 | [E30](./E30-rebuild-shapes-from-their-pieces.md) | Rebuild shapes from their pieces | draft |
| 18 | [E31](./E31-give-shapes-colors-and-strokes.md) | Give shapes colors and strokes | draft |
| 19 | [E32](./E32-refine-the-texts-lines-and-options.md) | Refine the text's lines and options | draft |
| 20 | [E33](./E33-choose-fonts-for-synoptic-views.md) | Choose fonts for synoptic views | draft |
| 21 | [E34](./E34-build-symbols-with-ports.md) | Build symbols with ports | draft |
| 22 | [E35](./E35-animate-parts-of-symbols.md) | Animate parts of symbols | draft |
| 23 | [E36](./E36-configure-business-types.md) | Configure business types | draft |
| 24 | [E37](./E37-publish-business-symbols-and-libraries.md) | Publish business symbols and libraries | draft |
| 25 | [E38](./E38-assemble-synoptic-views.md) | Assemble synoptic views | draft |
| 26 | [E39](./E39-connect-symbols-with-orthogonal-pipes.md) | Connect symbols with orthogonal pipes | draft |
| 27 | [E40](./E40-draw-and-reuse-static-drawings.md) | Draw and reuse static drawings | draft |
| 28 | [E41](./E41-open-and-save-files-from-the-web-page.md) | Open and save files from the web page | draft |
| 29 | [E42](./E42-import-svg-from-other-tools.md) | Import SVG from other tools | draft |
| 30 | [E43](./E43-tune-settings-and-keys.md) | Tune settings and keys | draft |
| 31 | [E44](./E44-edit-the-shape-tree-in-a-layers-panel.md) | Edit the shape tree in a layers panel | draft |
| 32 | [E45](./E45-design-the-editors-interface.md) | Design the editor's interface | draft |
| 33 | [E46](./E46-run-the-editor-as-a-desktop-application.md) | Run the editor as a desktop application | draft |
| 34 | [E47](./E47-explain-the-project-to-newcomers.md) | Explain the project to newcomers | draft |
| 35 | [E48](./E48-check-each-release.md) | Check each release | draft |

## Dropped

The epics not done before 2026-10-25, dropped by SP-033 (ADR-0100, decision 1), each naming in its file the new epics that took its content; E16's goes to the pro edition (`editions.md` §2).

| ID | Epic | Content now in | Status |
| --- | --- | --- | --- |
| [E18](./E18-transform-shapes-by-hand.md) | Transform shapes and points by hand | E21, E22, E23, E24, E25, E31 | dropped |
| [E19](./E19-snap-align-and-distribute.md) | Snap, align and distribute shapes | E26 | dropped |
| [E13](./E13-arrange-shapes-in-a-tree.md) | Arrange shapes in a tree and lay them out | E27, E28, E44 | dropped |
| [E03](./E03-combine-shapes.md) | Combine shapes without losing their numbers | E29, E30 | dropped |
| [E04](./E04-outline-shapes.md) | Give shapes colors and exact strokes | E31 | dropped |
| [E05](./E05-build-animatable-symbols.md) | Build animatable symbols | E34, E35 | dropped |
| [E06](./E06-configure-business-types.md) | Configure business types in cascade | E36 | dropped |
| [E07](./E07-publish-business-libraries.md) | Publish business symbol libraries | E37 | dropped |
| [E08](./E08-assemble-views.md) | Assemble synoptic views | E38 | dropped |
| [E09](./E09-route-pipes.md) | Connect symbols with orthogonal pipes | E39 | dropped |
| [E10](./E10-draw-static-drawings.md) | Draw and reuse static drawings | E40, E41 | dropped |
| [E11](./E11-store-and-share.md) | Store and share as files in a working folder | E41, E42, E46 | dropped |
| [E20](./E20-make-the-editor-easy-to-use.md) | Make the editor easy to use | E24, E32, E33, E43, E45 | dropped |
| [E12](./E12-release-a-trusted-product.md) | Release a product people can trust | E47, E48 | dropped |
| [E16](./E16-check-text-contrast.md) | Check the contrast of texts on their background | the pro edition | dropped |

## Next (SP-033, 2026-10-25)

E21, Resize shapes by their box, opens with its refinement spike from `develop` (ADR-0086): its first feature moves the helps still drawn in the drawing's SVG, the box frames and the points' handles, to the layer above (ADR-0099, decision 2).
