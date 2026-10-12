# Changelog

## 0.1.0 (2026-10-13)

A consolidated base after the first epic, E01 "Draw symbol shapes from numbers".

### Features

- Rectangles and regular polygons drawn from whole numbers, each corner rounded by its own radius, reduced to fit.
- A regular polygon anchored in its box at one of nine places, or stretched to fill it.
- Free shapes: points moved, added and deleted; the rectangle is one; decimals kept where nothing was typed.
- Any shape turned by any angle, with its turned and straight boxes, and edited while turned.
- The playground: a toolbar, grouped panels, the pipeline output, a reset, and a short guided test per feature.

### Tooling

- Checks run locally on every work branch; one CI, on the pull request into `main`.
- Metrics of the agent's work, a status line and a report of each session.
- A local library of the sources read, behind an MCP server.

The history before this version is summarized in [docs/process/history.md](./docs/process/history.md).
