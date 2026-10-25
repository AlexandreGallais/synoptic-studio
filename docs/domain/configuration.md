# Configuration

The Configurator gives symbols their business meaning, without touching their geometry.

## 1. Interfaces and properties

- **A property**: a name, a type and a default value; every property has one. `isRunning: boolean`, default `false`.
- **An interface**: a named set of properties, with rules driving the symbol's animations. Interface `running`: "if `isRunning`, part _body_ is green, otherwise grey".
- **Rules live in the Configurator**, not in the Symbol Editor: a symbol only exposes animatable parts ([symbols and views](./symbols-and-views.md) §3).
- **A property group**: a named list of properties used together; a property may belong to several. "Pop-up properties", "maintenance properties".

## 2. Configuration trees

- **Trees of nodes**, each carrying interfaces and property groups. Pump → Positive displacement pump → Gear pump.
- **A node brings its ancestors' options**: the deeper the node, the more options. "Gear pump" has the options of Pump, Positive displacement pump and Gear pump.
- **Trees combined**: a business symbol may also select a sub-tree of another tree. A pump adding "Alarms → Motor fault".
- **Defaults cascade**: a node's default values apply to everything selected below it. Pump's `ratedFlow` default of 10 holds for Gear pump.

## 3. Business types and business symbols

- **A business type** is a node of a configuration tree. "Gear pump".
- **A business symbol** is a symbol, the configuration selected (nodes or sub-trees) and a business name; it selects parts of the configuration, never parts of the symbol. "Gear pump" made of the pump symbol and the Gear pump node.
- **One symbol, several business symbols**: the same symbol serves different business symbols in different projects. The pump symbol is a pump in one project, a compressor in another.

## 4. Business libraries

- **A business library** groups the business symbols of a project; two projects may share symbols with different libraries. Project A's library holds "Gear pump"; project B's, built on the same symbol, "Compressor".

## 5. In the View Editor

- **Overrides**: an instance gets its configuration's defaults and may override them; a value bound to an animation shows the result live. A valve overridden to open is drawn open.
- **The pop-up** at run time lists the properties of the configured pop-up group, every line checked by default; an instance may uncheck and check lines again. A pump's pop-up hides `serialNumber`.
- **Hidden by default**: a property may be configured unchecked by default; it stays in the list and can be checked in the View Editor.
- **Never a new property**: every line comes from the configuration. A view cannot add `color` to a pump.

## 6. Storage

- **Files on the user's computer**, saved one by one: symbols, configurations, libraries and drawings; opened from the disk and saved by Save as from the web page first, in a working folder once the desktop application comes; no server, not even a local one, servers belonging to the pro edition ([editions](./editions.md)). `pump.json` saved into a chosen folder.
- **The drawing library** is shared between projects ([shapes](./shapes.md) §8).
