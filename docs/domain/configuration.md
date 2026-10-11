# Configuration

The Configurator gives symbols their business meaning, without touching their geometry. Source: the user's description of 2026-10-08.

## 1. Interfaces and properties

- A **property** = name, type, default value. Every property declared in the configuration has a default value.
- An **interface** = a named set of properties, plus **rules** driving the symbol's animations. Example: interface `running` with property `isRunning: boolean`; rule "if `isRunning`, part _body_ takes color _green_, otherwise _grey_".
- Rules are written in the Configurator, not in the Symbol Editor: the symbol only exposes animatable parts ([symbols-and-views.md](./symbols-and-views.md) §3).
- A **property group** = a named list of properties used together, e.g. "pop-up properties", "maintenance properties". A property may belong to several groups.

## 2. Configuration trees

- Configuration is organized as **trees** of nodes. Each node carries interfaces and property groups.
- Example: Pump → Positive displacement pump → Gear pump.
- Scope (Q14): selecting a node brings **its own options and those of all its ancestors**. The deeper the node, the more options: "Gear pump" has the options of Pump, Positive displacement pump and Gear pump.
- Several trees can be combined: a business symbol may also select a sub-tree of another tree, e.g. a pump adding "Alarms → Motor fault".
- Properties are part of the cascade: a node's defaults apply to everything selected below it.

## 3. Business types and business symbols

- A **business type** is a node of a configuration tree (e.g. "Gear pump").
- A **business symbol** = a symbol + the selected configuration (nodes or sub-trees) + a business name. It does not select parts of the symbol, it selects parts of the configuration.
- The same symbol can serve different business symbols in different projects (a pump symbol used for a pump in one project, for another device in another).

## 4. Business libraries

- A **business library** groups the business symbols of a project.
- Two projects can share symbols while having different business libraries.

## 5. In the View Editor

- An instance gets the default values of its configuration; the View Editor may **override** them per instance.
- Overriding a value bound to an animation shows the result **live** (e.g. a valve displayed open or closed by default).
- The pop-up shown at run time lists the properties of the configured pop-up group. Every line is checked by default; the instance may **uncheck** (and re-check) lines.
- A property may be configured as **hidden by default** (unchecked by default); it stays in the list and can be checked in the View Editor.
- The View Editor **never adds** a property: every line comes from the configuration.

## 6. Storage

- Symbols, configurations, libraries and drawings are **files in a working folder** on the user's computer, saved one by one (Q5). No server, not even a local one: servers and online services belong to the pro edition ([editions](./editions.md)).
- The drawing library is shared between projects ([shapes.md](./shapes.md) §8).
