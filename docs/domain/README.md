# Domain

The project's business rules: each rule stated with an example, kept true to the code.

## Vision

Four tools built on the same `editor` library, from geometry to business use:

| Tool | User | Does | Does not |
| --- | --- | --- | --- |
| **Symbol Editor** | symbol designer | draws a symbol's shapes with strict, numeric tools; places ports; declares the animatable parts | give business meaning or rules |
| **Configurator** | business configurator | builds configuration trees: interfaces, property groups, default values, rules driving the animations; defines business types and business libraries | change a symbol's geometry |
| **View Editor** | synoptic view designer | places business symbols by quarter turns, overrides default values with a live preview, chooses the pop-up's properties, draws pipes | add a property the configuration does not define; change a symbol's geometry |
| **Drawing tools** (in the View Editor) | synoptic view designer | draws static drawings with more freedom, saved into a drawing library shared between projects | parameters, animations |

The same symbols can serve different business libraries: the library gives a symbol its meaning for a project.

## Purpose

- **A product, not a demo**: a library and editors that teams drawing synoptic views can adopt with confidence.
- **Trust through quality**: every behavior specified, tested and documented.
- **Portable mathematics**: the library computes its own geometry, never through the SVG or DOM APIs, runs in any JavaScript engine, and can be transcribed into another language.
- **A finished free V1, then a paid pro edition** ([editions](./editions.md)).
- **Built by an agent**: a coding agent writes every change, the Product Owner reviews; outside input comes through issues.

## Principles

- **Schematic**: a synoptic view stays readable.
- **Logical**: every symbol shape is described by numbers — corners, sizes, radii, angles —, never by a hand gesture.
- **Orthogonal**: pipes are horizontal and vertical.
- **Integer**: every input value is an integer; derived geometry and animation values may not be.
- **Configured, not hand-made**: a view uses only what the configuration defines.

## Pages

| Page | Content |
| --- | --- |
| [Shapes](./shapes.md) | shape model, rotation, corner radius, shape tools, text, booleans, strokes, static drawings |
| [Symbols, views and pipes](./symbols-and-views.md) | symbols, ports, animations, views, pipes |
| [Configuration](./configuration.md) | configuration trees, interfaces, property groups, business types, libraries |
| [Interaction](./interaction.md) | tools, selection, moves, node editing, snapping, colors, the view, undo and redo |
| [Editions](./editions.md) | the free V1 and the pro edition |

## Glossary

| Term | Definition |
| --- | --- |
| Shape | closed contour of vertices with a radius per vertex (a round shape has none), drawn as a `<path>` |
| Vertex, node | point of a contour: integer when typed, fractional when derived or kept by a free shape from a conversion |
| Free shape | a contour stored as its corners (a point and its radius) in a box of their own, whose width and height stretch it without loss; the rectangle tool makes one, a regular polygon becomes one by a button |
| Round shape | a shape filling its box, a circle in a square box, an ellipse otherwise, drawn as four cubic sides; width, height, angle and position, no radius |
| Corner radius | radius of the arc rounding a vertex, one per vertex |
| Fillet | arc tangent to both edges of a vertex |
| Setback | distance from the vertex to the fillet's tangent point, `r / tan(θ/2)` |
| Tangent point | point where a fillet touches an edge |
| Requested radius | the radius typed and stored |
| Effective radius | the radius drawn after clamping, computed at each drawing, never stored |
| Spike | vertex where the contour turns back on itself; its fillet consumes it |
| Stroke | band of fixed width along the outline |
| Stroke alignment | `inner`, `center`, `outer` |
| Boolean operation | union, difference, intersection, exclusion, division, cut path |
| Shape builder | tool building regions from the edges and intersections of selected shapes |
| Symbol | group (`<g>`) of shapes, ports and animatable parts, without business meaning |
| Animation | change of a symbol's part driven by a property: color, blinking, opacity, visibility, partial fill, rotation |
| Port | point of a symbol where a pipe connects |
| Configuration tree | tree of configuration nodes carrying interfaces and property groups |
| Interface | named set of properties with rules driving a symbol's animations, such as `isRunning` turning a zone green |
| Property | typed value with a default, member of an interface or a property group |
| Property group | named list of properties used together, such as the pop-up's |
| Business type | node of a configuration tree, such as Gear pump |
| Business symbol | a symbol, the configuration selected and a business name, such as "gear pump" |
| Business library | the business symbols a project uses |
| Instance | occurrence of a business symbol in a view |
| View | assembly of instances, pipes and static drawings |
| Pipe | orthogonal connector between two ports |
| Waypoint | bend of a pipe |
| Static drawing | drawing without parameters made in the View Editor, reused through the drawing library |
| Drawing library | drawings shared between projects |
| Snapping | automatic attachment to a remarkable position |
| Bounding box | smallest axis-aligned rectangle containing an object |
| Position | top-left of a shape's box before it turns, stored with its angle; shown and typed as the top-left of its straight box |
| Straight box | bounding box of a drawn shape, rounded corners included; its top-left is the position shown |
| Turned box | a shape's box turned with it, shown as its selection |
| Stretched polygon | a regular polygon with "stretched" on: it touches the four sides of its box, its sides as even as possible |
| Anchor | place of a shape in its box, one of 9: a horizontal (left, center, right) and a vertical (top, center, bottom) alignment, `min`, `mid`, `max` in the model; not the stroke alignment, the text anchor, the Align command nor snapping's guides |
| Weight | thickness of a font's letters, an integer from 100 (thin) to 900 (black), 700 for bold |

## Open questions

A rule not settled yet: work that needs it stops and asks.

| # | Question |
| --- | --- |
| Q4 | the strokes of curved sides, and booleans on curved sides |
| Q9 | a corner radius on the vertices a boolean creates |
| Q24 | a `foreignObject` tool (such as an `iframe`) in a given frame: in the free V1 or the pro edition, against the rule of path-only drawing |
| Q25 | the corner radii of a layout box: one for the whole box, or one per corner |
| Q30 | when a symbol is saved, its shapes moved so the top-left of their bounding box is at (0, 0), or its centre, or left where they are |
| Q42 | font subsetting at export: only a text fixed at export could carry a subset; which texts are fixed, and who says so |
| Q43 | free positions: an option leaving a shape where it is dropped, off the integers |
