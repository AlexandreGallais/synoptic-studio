# Symbols, views and pipes

## 1. Symbol

- **A symbol** is a `<g>` group holding a shape tree (`interaction.md` §6), ports and animatable parts. A pump symbol: a circle, a triangle, two ports and a rotating impeller.
- **No business meaning**: interfaces, properties and rules come from the Configurator ([configuration](./configuration.md)). The same pump symbol serves a pump or a fan.
- **No `<symbol>` nor `<use>`**: the model carries the symbol's behavior; it is drawn as a `<g>`.
- **An origin**: an integer reference point, used to place it. Origin (0, 0).
- **Shapes turn by any integer angle** inside the Symbol Editor ([shapes](./shapes.md) §1). A blade turned by 30°.
- **Planned**:
  - **A clickable zone**: a rectangle laid over the symbol receives its clicks and sends its event, instead of the shapes, the designer choosing one or the other; parts may stick out of it. A valve's zone 40 × 40 while its handle sticks out.
  - **A shared status zone**: a rectangle on every symbol where a status common to all is shown alike, its drawing computed from the zone. The loss of a device's network link shown in the top-right zone.
  - **The saved position** of a symbol's shapes is an open question (Q30): moved so the top-left of their bounding box is at (0, 0), or its centre, or left as they are.

## 2. Ports (connection points)

| Property | Description | Example |
| --- | --- | --- |
| `id` | unique within the symbol | `inlet` |
| Position | integer point, relative to the symbol's origin | (0, 20) |
| Exit direction | `N`, `E`, `S`, `W`: the pipe leaves the port perpendicular to its edge; turns with the instance | `W` turned by a quarter turn is `N` |
| Count | free | a pump has 2, sometimes more |

## 3. Animations

An animatable part reacts to a value a configuration rule sets:

| Animation | Effect |
| --- | --- |
| Color | the part takes a color |
| Color blinking | the part alternates between colors |
| Opacity or visibility blinking | the part blinks by opacity or display |
| Visibility | the part is shown or hidden |
| Partial fill | the part is filled up to a level, as a tank |
| Rotation | the part turns by an angle remapped from the property |

- **Remapping**: an animation never reads a property raw; the property's range is remapped to the animation's, set per animation, so one property can drive any animation. A property from 0 to 100 driving a rotation of 0 to 90°: 50 turns it by 45°.

## 4. Synoptic view (View Editor)

- **Contents**: instances of business symbols, pipes, static drawings. A view of three pumps, a tank and their pipes.
- **On an instance**: an integer position; a rotation by quarter turns only, `q ∈ {0, 1, 2, 3}`, about the symbol's origin, its ports' directions turning with it; default values overridden with a live preview of the animations; pop-up lines checked or unchecked ([configuration](./configuration.md) §5). `q = 1` turns a pump by 90°.
- **Forbidden**: changing a symbol's geometry; adding a property the configuration does not define. A pump's circle cannot be resized in a view.
- **Duplicating** instances is allowed. A pump copied keeps its overrides.

## 5. Pipes

- **Orthogonal only**: horizontal and vertical segments; no diagonals. A pipe from (0, 0) to (100, 50) bends once or more.
- **Created** by a click on a port, then on another port. `pump1.outlet` to `tank.inlet`.
- **Routed automatically** at first, avoiding the symbols. A pipe goes around a tank in its way.
- **Attached**: a pipe stays on its ports and follows the symbols when they move. A pump moved by (20, 0) drags its pipe's end.
- **Edited** by dragging a segment perpendicular to its direction, its neighbours adapting. A horizontal segment dragged down by 10.
- **Rounded bends**, optional, by the shapes' corner radius. A bend of radius 8.
- **Conventions of industrial diagrams**: process lines horizontal and vertical, drawn cleanly; at a crossing the horizontal line is continuous and the vertical one broken; horizontal lines dominate, the main flow left to right; line weight by hierarchy, main process over secondary over utilities. A vertical pipe crossing a horizontal one shows a gap.

## 6. Storage

Files on the user's computer, saved one by one: from the web page first, in a working folder with the desktop application ([configuration](./configuration.md) §6).
