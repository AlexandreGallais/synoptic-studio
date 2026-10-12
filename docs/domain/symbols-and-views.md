# Symbols, views and pipes

## 1. Symbol

- A symbol = a `<g>` group containing a **shape tree** (see `interaction.md` §6), **ports** and **animatable parts**.
- A symbol has no business meaning: interfaces, properties and rules come from the Configurator ([configuration.md](./configuration.md)).
- No SVG `<symbol>` / `<use>` element: symbol behavior is carried by the model, rendering stays a `<g>`.
- Symbol origin: integer reference point for placement.
- Inside the Symbol Editor, shapes may be rotated by any integer angle in degrees ([shapes.md](./shapes.md) §1).
- **For later** (Product Owner, VAL-005 of F08, 2026-10-11; epic or feature to choose):
  - **Clickable zone**: a symbol may have a rectangle laid over it as the zone that receives the clicks and sends its event, instead of the shape itself; the designer chooses one or the other. Parts of a symbol may stick out of its working area while the click stays on the zone.
  - **Shared status zone**: a rectangle defined on every symbol where a status common to all symbols is shown the same way (for example the loss of the network link to a device), its drawing computed from that zone.
  - **Saved position**: when a symbol is saved, its shapes may be moved so that the top-left of their bounding box is at (0, 0), giving positive coordinates, integers when the points are; another choice would be its center at (0, 0) — open (Q30).

## 2. Ports (connection points)

| Property       | Description                                                                                                                                    |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | unique identifier within the symbol                                                                                                            |
| Position       | integer point, relative to the symbol origin                                                                                                   |
| Exit direction | `N`, `E`, `S`, `W` — needed by orthogonal routing (the pipe leaves the port perpendicularly to the edge); rotates with the instance (ADR-0008) |
| Count          | free; e.g. a pump usually has 2, sometimes more                                                                                                |

## 3. Animations

An animatable part of a symbol reacts to a value set by a configuration rule:

| Animation                     | Effect                                                         |
| ----------------------------- | -------------------------------------------------------------- |
| Color                         | the part takes a color                                         |
| Color blinking                | the part alternates between colors                             |
| Opacity / visibility blinking | the part blinks by opacity or display                          |
| Visibility                    | the part is shown or hidden                                    |
| Partial fill                  | the part is filled up to a level (e.g. a tank level)           |
| Rotation                      | the part rotates by an angle obtained by remapping (see below) |

### Remapping (Q13)

An animation does not read a property raw: the property's range is **remapped** to the animation's range. Example: a property from 0 to 100 drives a rotation remapped to 0–90°, 0–180° or 0–360°. The remap is set per animation, so that one property can drive any animation type.

## 4. Synoptic view (View Editor)

- Contains: instances of business symbols, pipes, static drawings.
- Allowed on an instance: position (integer), rotation by **quarter turns only** (ADR-0008), override of default property values (live preview of animations), checking / unchecking pop-up lines ([configuration.md](./configuration.md) §5).
- Forbidden: changing the symbol's geometry; adding a property not defined by the configuration.
- Duplicating instances: yes.
- Rotation: integer `q ∈ {0, 1, 2, 3}`, pivot = symbol origin; port directions rotate with the instance.

## 5. Pipes

### Drawing

- **Orthogonal only**: horizontal and vertical segments (ADR-0004).
- Creation: click a port → click another port.
- Initial path: automatic routing avoiding the symbols (`REF-WYBROW-2009`, `REF-MARRIOTT-2014`).
- The pipe stays attached to its ports and **follows the symbols when they move**.
- Manual editing: dragging a segment moves it **perpendicularly** to its direction (draw.io behavior); neighboring segments adapt.
- Rounded bends: optional, same mechanism as the corner radius of shapes.

### Conventions from industrial diagrams (P&ID)

| Convention                                                              | Source                             |
| ----------------------------------------------------------------------- | ---------------------------------- |
| Process lines horizontal and vertical, drawn cleanly                    | `REF-PID-LECTURE`                  |
| Crossing: the horizontal line is continuous, the vertical one is broken | `REF-PID-LECTURE`                  |
| Horizontal lines dominate; main flow from left to right                 | `REF-PID-LECTURE`, `REF-EDRAW-PID` |
| Line weight by hierarchy (main process > secondary > utilities)         | `REF-EDRAW-PID`                    |

Standards to consult to go further: `REF-ISO-10628`, `REF-ISA-5-1`, `REF-ISA-101`, `REF-TOGHRAEI`.

### Diagonals (45°)

Excluded in v1 (process diagram conventions, orthogonal routing algorithms). Can be reintroduced through a dedicated ADR.

## 6. Storage

Files in a working folder on the user's computer, saved one by one (Q5, [configuration.md](./configuration.md) §6).
