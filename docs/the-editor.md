# The editor

synoptic-studio is a set of editors for synoptic views — the schematic screens that show and drive an industrial plant: its pumps, valves, tanks and the pipes between them. Everything is drawn from numbers, so that every symbol is exact and every view stays readable.

## The editors

| Editor | Who uses it | What it does |
| --- | --- | --- |
| **Symbol Editor** | the symbol designer | draws a symbol's shapes from typed numbers, places its connection points and names the parts that will move or change color |
| **Configurator** | the business configurator | gives symbols their meaning: the properties of a pump or a valve, their default values and the rules that animate the symbol, such as "running turns the body green" |
| **View Editor** | the view designer | places the configured symbols on a view, sets their values, chooses what their pop-ups show, connects them by horizontal and vertical pipes, and adds free drawings such as frames and legends |

The same symbol can serve several projects: each project's library gives it its business meaning. The first edition is free and runs entirely on the user's computer, as a web page first, then as a desktop application; files are saved one by one, without any server.

## What one can draw

- **Rectangles and regular polygons**, from a triangle to a twelve-sided polygon, each corner rounded by its own radius.
- **Free shapes**, whose points are moved, added and deleted one by one, and bent into curves by handles.
- **Circles and ellipses**, filling their box.
- **Text**, measured with the real metrics of its font, in one line or wrapped in a box, ending with "…" when it is too long, its fonts embedded in the exported file.
- **Colors**, typed in hexadecimal or picked in a color picker, with their opacity.

Every size, position, radius and angle is a whole number typed in the panel; any shape can be turned by any angle. Later come groups, layouts, booleans, strokes, ports, animations and pipes, in the Product Owner's order of the epics.

## How one works on a drawing

- **Adding**: choose a tool, then click on the drawing to place a shape of a default size, or drag to draw its box; a text is typed right after the click.
- **Selecting**: a click picks the shape under the pointer — only where it is really painted —; Shift adds or removes a shape; a drag on an empty place draws a rectangle that selects what it touches, or with Alt what it encloses; Ctrl + A selects all, Escape none.
- **Moving**: drag the selected shapes, or nudge them with the arrow keys, by 1 or by 10 with Shift; they always land on whole numbers.
- **Editing**: the panel shows the values of the shape selected, or of several at once, where a value typed with a sign, such as `+10`, is added to each; in Points mode each point of a shape is edited on its own.
- **Undoing**: Ctrl + Z undoes each finished action — a move, a value typed, a shape added or deleted —, Ctrl + Shift + Z redoes it, and the list of steps goes back to any of them.
- **Viewing**: the wheel pans, Ctrl and the wheel or a pinch zoom about the pointer, Shift + 1 fits the whole drawing; the view never changes the drawing nor the exported file.

## How it is made

A coding agent builds the editors step by step, each change tested and checked; the Product Owner decides the rules and the priorities and tries each change in the [playground](https://alexandregallais.github.io/synoptic-studio/playground/). How the work is organised: [how the project works](./how-it-works.md).
