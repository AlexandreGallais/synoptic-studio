import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { defineConfig } from "vitepress";

import type { DefaultTheme } from "vitepress";

/** Base path of the published site (`/synoptic-studio/` on GitHub Pages), `/` locally. */
const BASE = process.env["DOCS_BASE"] ?? "/";

/** Root folder of the documentation site. */
const DOCS_ROOT = join(import.meta.dirname, "..");

/** Sidebar written by `tools/api-pages` next to the API pages (EN-072). */
const API_SIDEBAR_FILE = join(DOCS_ROOT, "api", "api-sidebar.json");

/** Name of a folder's index page in the repository (served as the folder's index). */
const FOLDER_INDEX = "README.md";

/**
 * A section of the site: a folder, its index first when it has one, then its pages, every page of
 * the folder or the ones named, in that order.
 */
type Section = {
  readonly collapsed: boolean;
  readonly folder: string;
  readonly overview: boolean;
  readonly pages: "all" | readonly string[];
  readonly title: string;
};

/** A group of sections, a heading of the sidebar and an entry of the navigation bar. */
type Group = {
  readonly sections: readonly Section[];
  readonly title: string;
};

/**
 * The site in reading order, for the agent first (REV-009): what the project is, how to work,
 * the tooling, then the records, grouped and last.
 */
const GROUPS: readonly Group[] = [
  {
    sections: [
      { collapsed: false, folder: "domain", overview: true, pages: "all", title: "Domain" },
    ],
    title: "The project",
  },
  {
    sections: [
      {
        collapsed: false,
        folder: "conventions",
        overview: true,
        pages: "all",
        title: "Conventions",
      },
      {
        collapsed: false,
        folder: "process",
        overview: true,
        pages: ["agile.md"],
        title: "Process",
      },
      { collapsed: false, folder: "backlog", overview: true, pages: "all", title: "Backlog" },
      { collapsed: true, folder: "backlog/epics", overview: true, pages: "all", title: "Epics" },
      {
        collapsed: true,
        folder: "backlog/features",
        overview: true,
        pages: "all",
        title: "Features",
      },
      { collapsed: false, folder: "research", overview: true, pages: "all", title: "Research" },
      { collapsed: true, folder: "playbook", overview: true, pages: "all", title: "Playbook" },
    ],
    title: "How to work",
  },
  {
    sections: [
      { collapsed: false, folder: "tooling", overview: true, pages: "all", title: "Tooling" },
      { collapsed: true, folder: "api", overview: true, pages: [], title: "API" },
    ],
    title: "Tooling",
  },
  {
    sections: [
      { collapsed: true, folder: "adr", overview: true, pages: "all", title: "Decisions" },
      {
        collapsed: true,
        folder: "research/notes",
        overview: true,
        pages: "all",
        title: "Research notes",
      },
      {
        collapsed: true,
        folder: "research/requests",
        overview: false,
        pages: "all",
        title: "Research requests",
      },
      {
        collapsed: true,
        folder: "derivations",
        overview: true,
        pages: "all",
        title: "Derivations",
      },
      {
        collapsed: true,
        folder: "",
        overview: false,
        pages: ["references.md"],
        title: "References",
      },
      { collapsed: true, folder: "guide", overview: true, pages: "all", title: "Guide" },
      {
        collapsed: true,
        folder: "process/retrospectives",
        overview: true,
        pages: "all",
        title: "Retrospectives",
      },
      {
        collapsed: true,
        folder: "process",
        overview: false,
        pages: ["journal.md", "improvements.md", "feedback.md", "tool-incidents.md", "history.md"],
        title: "Logs",
      },
    ],
    title: "Records",
  },
];

/** The address of the playground, as of the last merge into `main`. */
const PLAYGROUND = "https://alexandregallais.github.io/synoptic-studio/playground/";

/**
 * Title of a Markdown page: its first level-1 heading, or its file name.
 *
 * @param path - absolute path of the page
 * @returns text shown in the sidebar for this page
 */
function pageTitle(path: string): string {
  const heading = /^# (?<title>.+)$/mu.exec(readFileSync(path, "utf8"));

  return heading?.groups?.["title"] ?? path;
}

/**
 * The pages of a section: every Markdown page of its folder sorted by file name, its index and
 * the templates left out, or the ones it names, in its order.
 *
 * @param section - section of the site
 * @returns the file names of its pages
 */
function sectionPages(section: Section): readonly string[] {
  if (section.pages !== "all") {
    return section.pages;
  }

  return readdirSync(join(DOCS_ROOT, section.folder))
    .filter((name) => name.endsWith(".md") && name !== FOLDER_INDEX && !name.startsWith("_"))
    .toSorted((left, right) => left.localeCompare(right, "en"));
}

/**
 * The address of a section: its index, or its first page when it has none.
 *
 * @param section - section of the site
 * @returns the link of its first entry
 */
function sectionLink(section: Section): string {
  const prefix = section.folder === "" ? "" : `/${section.folder}`;

  return section.overview
    ? `${prefix}/`
    : `${prefix}/${(sectionPages(section)[0] ?? "").slice(0, -".md".length)}`;
}

/**
 * Sidebar entries of a section: its index first when it has one, then its pages; the API's from
 * the sidebar `tools/api-pages` writes.
 *
 * @param section - section of the site
 * @returns sidebar items linking to its pages
 */
function sectionItems(section: Section): DefaultTheme.SidebarItem[] {
  const prefix = section.folder === "" ? "" : `/${section.folder}`;
  const overview = section.overview ? [{ link: `${prefix}/`, text: "Overview" }] : [];
  const pages = sectionPages(section).map((name) => ({
    link: `${prefix}/${name.slice(0, -".md".length)}`,
    text: pageTitle(join(DOCS_ROOT, section.folder, name)),
  }));

  return section.folder === "api" ? [...overview, ...apiItems()] : [...overview, ...pages];
}

/**
 * Whether a parsed JSON value is a list of sidebar items (the generated API sidebar).
 *
 * @param value - parsed JSON
 * @returns `true` for an array of objects with a `text` string
 */
function isSidebar(value: unknown): value is DefaultTheme.SidebarItem[] {
  return (
    Array.isArray(value) &&
    value.every((item: unknown) => typeof item === "object" && item !== null && "text" in item)
  );
}

/**
 * API sidebar written by `tools/api-pages`, empty before the first `npm run docs:api`.
 *
 * @returns sidebar items of the API reference
 */
function apiItems(): DefaultTheme.SidebarItem[] {
  const parsed: unknown = existsSync(API_SIDEBAR_FILE)
    ? JSON.parse(readFileSync(API_SIDEBAR_FILE, "utf8"))
    : [];

  return isSidebar(parsed) ? parsed : [];
}

/**
 * VitePress configuration of the documentation site.
 *
 * @see docs/tooling/documentation-site.md
 */
export default defineConfig({
  base: BASE,
  cleanUrls: true,
  description: "Integer, orthogonal, path-only SVG editing library for synoptic-view symbols.",
  lang: "en",
  lastUpdated: false,
  srcExclude: ["**/_*.md"],
  rewrites: {
    ":section/:subsection/README.md": ":section/:subsection/index.md",
    ":section/README.md": ":section/index.md",
  },
  themeConfig: {
    nav: [
      ...GROUPS.map((group) => ({
        items: group.sections.map((section) => ({
          link: sectionLink(section),
          text: section.title,
        })),
        text: group.title,
      })),
      { link: PLAYGROUND, text: "Playground" },
    ],
    outline: "deep",
    search: { provider: "local" },
    sidebar: GROUPS.map((group) => ({
      items: group.sections.map((section) => ({
        collapsed: section.collapsed,
        items: sectionItems(section),
        text: section.title,
      })),
      text: group.title,
    })),
  },
  title: "editor",
});
