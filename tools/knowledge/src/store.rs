//! The store of the project's records (research note 0035, The database): one SQLite file shared
//! by every branch and worktree, through `rusqlite` as the library's store (`REF-RUSQLITE`). Each
//! record is a row of its table, its text cut into rows of an item table (schema 6), every text
//! column searched by one FTS5 index, `records_index`, kept by triggers
//! (`REF-SQLITE-FTS5`). Two worktrees write at once: the file in WAL (`REF-SQLITE-WAL`), each write
//! in `BEGIN IMMEDIATE` (`REF-SQLITE-TRANSACTION`), a second writer waiting within rusqlite's busy
//! timeout (`REF-RUSQLITE-CONNECTION`, `REF-SQLITE-BUSY-TIMEOUT`). The schema's number is
//! `PRAGMA user_version` (`REF-SQLITE-PRAGMA`); its changes add a table, a column or a wider key,
//! never rename nor drop a column, so that an older binary still reads a newer file, and refuses
//! to write to it.

use std::fs;
use std::path::Path;

use library::store::make_library_tables;
use rusqlite::{Connection, Transaction, TransactionBehavior};

/// The number of the schema this binary writes, `PRAGMA user_version` of the file.
pub const SCHEMA_VERSION: i64 = 6;

/// The tables the tool keeps itself, which `knowledge sql` never drops nor rewrites: the
/// documents and what is cut from them, the metrics, the backlog and the registry (EN-154), what
/// moved from the Markdown and the library (schema 5), the records' items, the agent's tables the
/// full-text index reads and that index (schema 6).
pub const CORE_TABLES: [&str; 47] = [
    "documents",
    "fields",
    "sections",
    "sections_index",
    "links",
    "calls",
    "turns",
    "tables_registry",
    "epics",
    "features",
    "stories",
    "criteria",
    "tasks",
    "decisions",
    "notes",
    "derivations",
    "sources",
    "journal",
    "improvements",
    "feedback",
    "incidents",
    "retrospectives",
    "memories",
    "tool_uses",
    "archives",
    "saves",
    "sessions",
    "messages",
    "commands",
    "transcript_files",
    "rules",
    "pages",
    "chunks",
    "project_uses",
    "story_items",
    "feature_items",
    "epic_items",
    "decision_items",
    "retrospective_items",
    "rule_items",
    "derivation_steps",
    "note_findings",
    "conventions",
    "method_rules",
    "preferences",
    "tools",
    "records_index",
];

/// The core tables `knowledge sql` writes to by no statement at all: the documents and what is
/// cut from them, the memories of the schemas before 6, are written by `sync` alone, so that a
/// section never leaves its full-text index behind; the library's pages and their index, written
/// by the library.
pub const WRITTEN_BY_THE_TOOL: [&str; 7] = [
    "documents",
    "fields",
    "sections",
    "sections_index",
    "links",
    "pages",
    "chunks",
];

/// The tables of schema 1 (research note 0035, Its tables). A document is any former file or
/// record, cut into sections at its headings; `sections_index` indexes the sections without a copy
/// of their text, kept by the three triggers (`REF-SQLITE-FTS5`, §4.4.3); `calls` and `turns` take
/// the fields of the hooks' `tool-calls.jsonl` and `turns.jsonl` under their names, a field an older
/// line lacks left null.
const SCHEMA_1: &str = "
  CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY, kind TEXT NOT NULL, title TEXT NOT NULL, epic TEXT NOT NULL,
    feature TEXT NOT NULL, status TEXT NOT NULL, date TEXT NOT NULL, branch TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('draft', 'merged', 'dropped')),
    private INTEGER NOT NULL CHECK (private IN (0, 1)), path TEXT NOT NULL
  );
  CREATE TABLE IF NOT EXISTS fields (
    document TEXT NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    key TEXT NOT NULL, value TEXT NOT NULL
  );
  CREATE INDEX IF NOT EXISTS fields_by_key ON fields (key, value);
  CREATE TABLE IF NOT EXISTS sections (
    id INTEGER PRIMARY KEY,
    document TEXT NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    position INTEGER NOT NULL, heading TEXT NOT NULL, text TEXT NOT NULL,
    UNIQUE (document, position)
  );
  CREATE VIRTUAL TABLE IF NOT EXISTS sections_index USING fts5(
    heading, text, content = 'sections', content_rowid = 'id', tokenize = 'porter unicode61'
  );
  CREATE TRIGGER IF NOT EXISTS sections_insert AFTER INSERT ON sections BEGIN
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
  CREATE TRIGGER IF NOT EXISTS sections_delete AFTER DELETE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
  END;
  CREATE TRIGGER IF NOT EXISTS sections_update AFTER UPDATE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
  CREATE TABLE IF NOT EXISTS links (
    \"from\" TEXT NOT NULL REFERENCES documents (id) ON DELETE CASCADE, \"to\" TEXT NOT NULL,
    PRIMARY KEY (\"from\", \"to\")
  );
  CREATE TABLE IF NOT EXISTS calls (
    toolUseId TEXT UNIQUE, time TEXT, startedAt INTEGER, durationMs INTEGER, session TEXT,
    branch TEXT, step TEXT, agent TEXT, agentId TEXT, tool TEXT, summary TEXT, description TEXT,
    isBackground INTEGER, inputChars INTEGER, outputChars INTEGER, isOk INTEGER, error TEXT,
    permissionMode TEXT, turnStart INTEGER, turnElapsedMs INTEGER, turnTokens INTEGER,
    turnOutputTokens INTEGER, contextPercent INTEGER, model TEXT, pendingAgents INTEGER,
    pendingCommands INTEGER, snapshotAgeMs INTEGER
  );
  CREATE TABLE IF NOT EXISTS turns (
    session TEXT NOT NULL, start INTEGER NOT NULL, wallMs INTEGER, activeMs INTEGER,
    tokens INTEGER, outputTokens INTEGER, calls INTEGER, failures INTEGER, commits INTEGER,
    PRIMARY KEY (session, start)
  );
";

/// Schema 2 keys a document by its id and its branch, so that a branch writes its draft of an id
/// another branch holds (research note 0035, The rows of a branch): the four tables of the
/// documents rebuilt by SQLite's procedure for a changed key, new tables made, filled, the old
/// dropped and the new renamed (`REF-SQLITE-ALTER-TABLE`, §8), each row and each section's rowid
/// kept, so that the full-text index still points at its sections; the children of a document
/// take its branch. No column is renamed nor dropped: a query of schema 1 still reads the file.
const SCHEMA_2: &str = "
  CREATE TABLE documents_2 (
    id TEXT NOT NULL, kind TEXT NOT NULL, title TEXT NOT NULL, epic TEXT NOT NULL,
    feature TEXT NOT NULL, status TEXT NOT NULL, date TEXT NOT NULL, branch TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('draft', 'merged', 'dropped')),
    private INTEGER NOT NULL CHECK (private IN (0, 1)), path TEXT NOT NULL,
    PRIMARY KEY (id, branch)
  );
  INSERT INTO documents_2 (id, kind, title, epic, feature, status, date, branch, state, private, path)
    SELECT id, kind, title, epic, feature, status, date, branch, state, private, path
    FROM documents ORDER BY rowid;
  CREATE TABLE fields_2 (
    document TEXT NOT NULL, branch TEXT NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL,
    FOREIGN KEY (document, branch) REFERENCES documents (id, branch) ON DELETE CASCADE
  );
  INSERT INTO fields_2 (document, branch, key, value)
    SELECT fields.document, documents.branch, fields.key, fields.value
    FROM fields JOIN documents ON documents.id = fields.document ORDER BY fields.rowid;
  CREATE TABLE sections_2 (
    id INTEGER PRIMARY KEY, document TEXT NOT NULL, branch TEXT NOT NULL,
    position INTEGER NOT NULL, heading TEXT NOT NULL, text TEXT NOT NULL,
    UNIQUE (document, branch, position),
    FOREIGN KEY (document, branch) REFERENCES documents (id, branch) ON DELETE CASCADE
  );
  INSERT INTO sections_2 (id, document, branch, position, heading, text)
    SELECT sections.id, sections.document, documents.branch, sections.position, sections.heading,
      sections.text
    FROM sections JOIN documents ON documents.id = sections.document;
  CREATE TABLE links_2 (
    \"from\" TEXT NOT NULL, branch TEXT NOT NULL, \"to\" TEXT NOT NULL,
    PRIMARY KEY (\"from\", branch, \"to\"),
    FOREIGN KEY (\"from\", branch) REFERENCES documents (id, branch) ON DELETE CASCADE
  );
  INSERT INTO links_2 (\"from\", branch, \"to\")
    SELECT links.\"from\", documents.branch, links.\"to\"
    FROM links JOIN documents ON documents.id = links.\"from\";
  DROP TABLE links;
  DROP TABLE sections;
  DROP TABLE fields;
  DROP TABLE documents;
  ALTER TABLE documents_2 RENAME TO documents;
  ALTER TABLE fields_2 RENAME TO fields;
  ALTER TABLE sections_2 RENAME TO sections;
  ALTER TABLE links_2 RENAME TO links;
  CREATE INDEX fields_by_key ON fields (key, value);
  CREATE INDEX fields_by_document ON fields (document, branch);
  CREATE TRIGGER sections_insert AFTER INSERT ON sections BEGIN
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
  CREATE TRIGGER sections_delete AFTER DELETE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
  END;
  CREATE TRIGGER sections_update AFTER UPDATE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
";

/// Schema 3 keys a document by its id, its branch and its state, so that a branch writes its new
/// draft of an id while its merged row of it stays, read by every other branch until that draft is
/// merged (AUD-050): the four tables rebuilt as schema 2 rebuilt them (`REF-SQLITE-ALTER-TABLE`,
/// §8), each row and each section's rowid kept; the children of a document take its state, and
/// follow it when it changes, by `ON UPDATE CASCADE` on their composite key
/// (`REF-SQLITE-FOREIGN-KEYS`, §4.1, §4.3). No column is renamed nor dropped.
const SCHEMA_3: &str = "
  CREATE TABLE documents_3 (
    id TEXT NOT NULL, kind TEXT NOT NULL, title TEXT NOT NULL, epic TEXT NOT NULL,
    feature TEXT NOT NULL, status TEXT NOT NULL, date TEXT NOT NULL, branch TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('draft', 'merged', 'dropped')),
    private INTEGER NOT NULL CHECK (private IN (0, 1)), path TEXT NOT NULL,
    PRIMARY KEY (id, branch, state)
  );
  INSERT INTO documents_3 (id, kind, title, epic, feature, status, date, branch, state, private, path)
    SELECT id, kind, title, epic, feature, status, date, branch, state, private, path
    FROM documents ORDER BY rowid;
  CREATE TABLE fields_3 (
    document TEXT NOT NULL, branch TEXT NOT NULL, state TEXT NOT NULL, key TEXT NOT NULL,
    value TEXT NOT NULL,
    FOREIGN KEY (document, branch, state) REFERENCES documents (id, branch, state)
      ON DELETE CASCADE ON UPDATE CASCADE
  );
  INSERT INTO fields_3 (document, branch, state, key, value)
    SELECT fields.document, fields.branch, documents.state, fields.key, fields.value
    FROM fields JOIN documents
      ON documents.id = fields.document AND documents.branch = fields.branch
    ORDER BY fields.rowid;
  CREATE TABLE sections_3 (
    id INTEGER PRIMARY KEY, document TEXT NOT NULL, branch TEXT NOT NULL, state TEXT NOT NULL,
    position INTEGER NOT NULL, heading TEXT NOT NULL, text TEXT NOT NULL,
    UNIQUE (document, branch, state, position),
    FOREIGN KEY (document, branch, state) REFERENCES documents (id, branch, state)
      ON DELETE CASCADE ON UPDATE CASCADE
  );
  INSERT INTO sections_3 (id, document, branch, state, position, heading, text)
    SELECT sections.id, sections.document, sections.branch, documents.state, sections.position,
      sections.heading, sections.text
    FROM sections JOIN documents
      ON documents.id = sections.document AND documents.branch = sections.branch;
  CREATE TABLE links_3 (
    \"from\" TEXT NOT NULL, branch TEXT NOT NULL, state TEXT NOT NULL, \"to\" TEXT NOT NULL,
    PRIMARY KEY (\"from\", branch, state, \"to\"),
    FOREIGN KEY (\"from\", branch, state) REFERENCES documents (id, branch, state)
      ON DELETE CASCADE ON UPDATE CASCADE
  );
  INSERT INTO links_3 (\"from\", branch, state, \"to\")
    SELECT links.\"from\", links.branch, documents.state, links.\"to\"
    FROM links JOIN documents
      ON documents.id = links.\"from\" AND documents.branch = links.branch;
  DROP TABLE links;
  DROP TABLE sections;
  DROP TABLE fields;
  DROP TABLE documents;
  ALTER TABLE documents_3 RENAME TO documents;
  ALTER TABLE fields_3 RENAME TO fields;
  ALTER TABLE sections_3 RENAME TO sections;
  ALTER TABLE links_3 RENAME TO links;
  CREATE INDEX fields_by_key ON fields (key, value);
  CREATE INDEX fields_by_document ON fields (document, branch, state);
  CREATE TRIGGER sections_insert AFTER INSERT ON sections BEGIN
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
  CREATE TRIGGER sections_delete AFTER DELETE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
  END;
  CREATE TRIGGER sections_update AFTER UPDATE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
";

/// Schema 4 holds the backlog in tables of its own, shaped for the agent's questions — which
/// stories of a feature are open, which story a branch works on, which criteria a done feature
/// holds — rather than as files (E49, EN-154): `epics`, `features`, `stories`, their `tasks` and
/// the features' numbered `criteria`, each row's `branch` the branch whose work it is and its
/// `state` that branch's, `draft` until merged; and `tables_registry`, one row per table and what
/// it holds, every table of the file listed there. Only tables and indexes are added: an older
/// binary still reads the file.
const SCHEMA_4: &str = "
  CREATE TABLE IF NOT EXISTS tables_registry (name TEXT PRIMARY KEY, purpose TEXT NOT NULL);
  CREATE TABLE IF NOT EXISTS epics (
    id TEXT PRIMARY KEY, title TEXT NOT NULL, status TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0, topic TEXT NOT NULL DEFAULT '',
    branch TEXT NOT NULL DEFAULT '',
    state TEXT NOT NULL DEFAULT 'draft' CHECK (state IN ('draft', 'merged', 'dropped'))
  );
  CREATE TABLE IF NOT EXISTS features (
    id TEXT PRIMARY KEY, epic TEXT NOT NULL, title TEXT NOT NULL, status TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0, topic TEXT NOT NULL DEFAULT '',
    criteria TEXT NOT NULL DEFAULT '', branch TEXT NOT NULL DEFAULT '',
    state TEXT NOT NULL DEFAULT 'draft' CHECK (state IN ('draft', 'merged', 'dropped'))
  );
  CREATE INDEX IF NOT EXISTS features_by_epic ON features (epic, position);
  CREATE TABLE IF NOT EXISTS stories (
    id TEXT PRIMARY KEY, epic TEXT NOT NULL, feature TEXT NOT NULL DEFAULT '', kind TEXT NOT NULL,
    title TEXT NOT NULL, status TEXT NOT NULL, points INTEGER, position INTEGER NOT NULL DEFAULT 0,
    topic TEXT NOT NULL DEFAULT '', criteria TEXT NOT NULL DEFAULT '',
    tasks TEXT NOT NULL DEFAULT '', test_card TEXT NOT NULL DEFAULT '',
    question TEXT NOT NULL DEFAULT '', branch TEXT NOT NULL DEFAULT '',
    state TEXT NOT NULL DEFAULT 'draft' CHECK (state IN ('draft', 'merged', 'dropped'))
  );
  CREATE INDEX IF NOT EXISTS stories_by_feature ON stories (feature, position);
  CREATE INDEX IF NOT EXISTS stories_by_epic ON stories (epic, status);
  CREATE INDEX IF NOT EXISTS stories_by_status ON stories (status);
  CREATE INDEX IF NOT EXISTS stories_by_branch ON stories (branch, state);
  CREATE TABLE IF NOT EXISTS tasks (
    story TEXT NOT NULL, number INTEGER NOT NULL, title TEXT NOT NULL,
    done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1)), PRIMARY KEY (story, number)
  );
  CREATE TABLE IF NOT EXISTS criteria (
    id TEXT PRIMARY KEY, feature TEXT NOT NULL, number INTEGER NOT NULL, text TEXT NOT NULL,
    struck INTEGER NOT NULL DEFAULT 0 CHECK (struck IN (0, 1)), retired TEXT NOT NULL DEFAULT ''
  );
  CREATE INDEX IF NOT EXISTS criteria_by_feature ON criteria (feature, number);
  INSERT OR IGNORE INTO tables_registry (name, purpose) VALUES
    ('documents', 'one row per document a branch wrote, its id, branch and state the key: the memories of schemas 1 to 5, read by nothing since schema 6'),
    ('fields', 'a document''s front-matter fields, by key and value'),
    ('sections', 'a document cut at its headings'),
    ('sections_index', 'FTS5 over sections, kept by three triggers'),
    ('links', 'the ids a document cites'),
    ('calls', 'a tool call of tool-calls.jsonl'),
    ('turns', 'a turn of turns.jsonl'),
    ('tables_registry', 'every table of the file and what it holds'),
    ('epics', 'one row per epic: its title, status, the Product Owner''s order (position), topic and vision; other facts in epic_items'),
    ('features', 'one row per feature: its epic, order, status, topic, criteria text and hypothesis; criteria in criteria, stories in stories, other facts in feature_items'),
    ('stories', 'one row per story: its epic, feature, kind, status, points, order, topic, summary, criteria, tasks, test card, question; its other facts in story_items; branch and state of the work'),
    ('tasks', 'a story''s tasks, numbered, done or not'),
    ('criteria', 'a feature''s numbered criteria, struck through or retired with a reason; exported for the CI''s trace');
";

/// Schema 5 holds what moved out of the repository's Markdown (E49, block 2 of the move): the
/// decisions, the research notes and requests, the derivations and the sources, each a row of
/// its own beside its document for search; the logs as rows, one per entry; the retrospectives,
/// the agent's memories, the uses of the tools, the archives' bundles, the saves, and what a
/// session's transcript holds that serves — its times and tokens, its commands and the Product
/// Owner's messages — rather than the transcript itself. The library's tables join the file by
/// the library's own function (`library::store::make_library_tables`), run after this batch. Only
/// tables and indexes are added: an older binary still reads the file.
const SCHEMA_5: &str = "
  CREATE TABLE IF NOT EXISTS decisions (
    id TEXT PRIMARY KEY, title TEXT NOT NULL, status TEXT NOT NULL, date TEXT NOT NULL,
    principle TEXT NOT NULL
  );
  CREATE INDEX IF NOT EXISTS decisions_by_status ON decisions (status);
  CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY, kind TEXT NOT NULL, title TEXT NOT NULL, status TEXT NOT NULL,
    date TEXT NOT NULL
  );
  CREATE TABLE IF NOT EXISTS derivations (
    id TEXT PRIMARY KEY, title TEXT NOT NULL, used_by TEXT NOT NULL
  );
  CREATE TABLE IF NOT EXISTS sources (
    id TEXT PRIMARY KEY, title TEXT NOT NULL, url TEXT NOT NULL, used_for TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('verified', 'unverified')), verified TEXT NOT NULL,
    topic TEXT NOT NULL, notes TEXT NOT NULL
  );
  CREATE INDEX IF NOT EXISTS sources_by_topic ON sources (topic);
  CREATE TABLE IF NOT EXISTS journal (
    id INTEGER PRIMARY KEY, date TEXT NOT NULL, session TEXT NOT NULL, tag TEXT NOT NULL,
    text TEXT NOT NULL, UNIQUE (date, session, text)
  );
  CREATE INDEX IF NOT EXISTS journal_by_tag ON journal (tag, date);
  CREATE TABLE IF NOT EXISTS improvements (
    id INTEGER PRIMARY KEY, date TEXT NOT NULL, origin TEXT NOT NULL, item TEXT NOT NULL,
    kind TEXT NOT NULL, UNIQUE (date, item)
  );
  CREATE TABLE IF NOT EXISTS feedback (
    id INTEGER PRIMARY KEY, date TEXT NOT NULL, happened TEXT NOT NULL, should_have TEXT NOT NULL,
    prevented_by TEXT NOT NULL, UNIQUE (date, happened)
  );
  CREATE TABLE IF NOT EXISTS incidents (
    id INTEGER PRIMARY KEY, date TEXT NOT NULL, tool TEXT NOT NULL, used TEXT NOT NULL,
    failed TEXT NOT NULL, instead TEXT NOT NULL, UNIQUE (date, tool)
  );
  CREATE TABLE IF NOT EXISTS retrospectives (
    id TEXT PRIMARY KEY, epic TEXT NOT NULL, title TEXT NOT NULL
  );
  CREATE TABLE IF NOT EXISTS memories (
    name TEXT PRIMARY KEY, description TEXT NOT NULL, type TEXT NOT NULL,
    private INTEGER NOT NULL DEFAULT 1 CHECK (private IN (0, 1)), modified TEXT NOT NULL,
    text TEXT NOT NULL
  );
  CREATE TABLE IF NOT EXISTS tool_uses (
    tool TEXT, subcommand TEXT, worktree TEXT, start TEXT, durationMs INTEGER, exitCode INTEGER,
    UNIQUE (tool, subcommand, worktree, start)
  );
  CREATE TABLE IF NOT EXISTS archives (
    name TEXT PRIMARY KEY, size INTEGER NOT NULL, modified INTEGER NOT NULL, bytes BLOB NOT NULL
  );
  CREATE TABLE IF NOT EXISTS saves (at TEXT PRIMARY KEY);
  CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY, start TEXT NOT NULL, end TEXT NOT NULL, wallMs INTEGER NOT NULL,
    inputTokens INTEGER NOT NULL, outputTokens INTEGER NOT NULL, messages INTEGER NOT NULL,
    commands INTEGER NOT NULL
  );
  CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY, session TEXT NOT NULL, time TEXT NOT NULL, text TEXT NOT NULL
  );
  CREATE INDEX IF NOT EXISTS messages_by_session ON messages (session, time);
  CREATE TABLE IF NOT EXISTS commands (
    id TEXT PRIMARY KEY, session TEXT NOT NULL, agent TEXT NOT NULL, time TEXT NOT NULL,
    command TEXT NOT NULL, description TEXT NOT NULL
  );
  CREATE INDEX IF NOT EXISTS commands_by_session ON commands (session, time);
  CREATE TABLE IF NOT EXISTS rules (
    id TEXT PRIMARY KEY, page TEXT NOT NULL, section TEXT NOT NULL, name TEXT NOT NULL,
    rule TEXT NOT NULL, example TEXT NOT NULL, position INTEGER NOT NULL
  );
  CREATE INDEX IF NOT EXISTS rules_by_section ON rules (section, position);
  CREATE TABLE IF NOT EXISTS transcript_files (
    path TEXT PRIMARY KEY, size INTEGER NOT NULL, modified INTEGER NOT NULL
  );
  INSERT OR IGNORE INTO tables_registry (name, purpose) VALUES
    ('decisions', 'one row per ADR: title, status, date, principle, decision, reason, consequences, refs, status_note, replaces, replaced_by, removed_reason, changed; other sections in decision_items; edited in place'),
    ('notes', 'one row per research note (NOTE-), research request (REQUEST-) or research page (PAGE-): kind, title, status, date; its findings in note_findings'),
    ('derivations', 'one row per derivation, DERIV-<name>: its title and the stories using it; its steps in derivation_steps'),
    ('sources', 'the bibliography, one row per REF- id: title, address, use, verified or not and when, topic, reading notes'),
    ('journal', 'one row per entry of the journal: its session''s date and heading, its tag, its text'),
    ('improvements', 'one row per idea, irritant or lesson: date, from whom, item, kind'),
    ('feedback', 'one row per critique of the agent''s work: what happened, what it should have done, what prevents it'),
    ('incidents', 'one row per tool incident: date, tool and what for, how used, why it failed, what was done instead'),
    ('retrospectives', 'one row per epic retrospective (RETRO-): its epic and title; its items in retrospective_items'),
    ('memories', 'the agent''s memories of Claude Code, one per file, private'),
    ('tool_uses', 'a use of an agent tool, of tool-uses.jsonl'),
    ('archives', 'a Git bundle of a branch archived, its bytes'),
    ('saves', 'the instant of each save of the records'),
    ('sessions', 'one row per session''s transcript: its first and last instants, tokens, the Product Owner''s messages and the commands counted'),
    ('messages', 'the Product Owner''s own messages, verbatim, private'),
    ('commands', 'each Bash command of a transcript, the main thread''s or an agent''s'),
    ('rules', 'one row per business rule of the domain''s pages: its part (RULE-<page>-<anchor>), name, rule and example'),
    ('transcript_files', 'each transcript read, its size and time, so that an unchanged one is not read again'),
    ('pages', 'the library: one row per page read, its Markdown'),
    ('chunks', 'the library: FTS5 over the sections of its pages'),
    ('project_uses', 'the library: each use of a page, by tool and project');
";

/// The migrations of a batch each, the one of index `n` bringing a file of schema `n` up to
/// `n + 1`: a new file runs them all, then [`schema_6`].
pub const MIGRATIONS: [&str; 5] = [SCHEMA_1, SCHEMA_2, SCHEMA_3, SCHEMA_4, SCHEMA_5];

/// Schema 6's tables (E49, AUD-057): each record's text cut into rows of its own, a section, a
/// kind (`paragraph`, `bullet`, a table's `row`, `code`) and the item, rather than a Markdown
/// copy; and the agent's tables the full-text index reads, made by `knowledge sql` on the records
/// before schema 6, so that a new file holds them too.
const SCHEMA_6_TABLES: &str = "
  CREATE TABLE IF NOT EXISTS story_items (story TEXT NOT NULL, section TEXT NOT NULL, n INTEGER NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), item TEXT NOT NULL, PRIMARY KEY (story, n));
  CREATE TABLE IF NOT EXISTS feature_items (feature TEXT NOT NULL, section TEXT NOT NULL, n INTEGER NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), item TEXT NOT NULL, PRIMARY KEY (feature, n));
  CREATE TABLE IF NOT EXISTS epic_items (epic TEXT NOT NULL, section TEXT NOT NULL, n INTEGER NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), item TEXT NOT NULL, PRIMARY KEY (epic, n));
  CREATE TABLE IF NOT EXISTS decision_items (decision TEXT NOT NULL, section TEXT NOT NULL, n INTEGER NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), item TEXT NOT NULL, PRIMARY KEY (decision, n));
  CREATE TABLE IF NOT EXISTS retrospective_items (retrospective TEXT NOT NULL, section TEXT NOT NULL, n INTEGER NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), item TEXT NOT NULL, PRIMARY KEY (retrospective, n));
  CREATE TABLE IF NOT EXISTS rule_items (page TEXT NOT NULL, n INTEGER NOT NULL, section TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), item TEXT NOT NULL, PRIMARY KEY (page, n));
  CREATE TABLE IF NOT EXISTS derivation_steps (derivation TEXT NOT NULL, n INTEGER NOT NULL, section TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), statement TEXT NOT NULL, source TEXT NOT NULL, PRIMARY KEY (derivation, n));
  CREATE TABLE IF NOT EXISTS note_findings (note TEXT NOT NULL, n INTEGER NOT NULL, section TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('paragraph','bullet','row','code')), finding TEXT NOT NULL, source TEXT NOT NULL, PRIMARY KEY (note, n));
  CREATE TABLE IF NOT EXISTS conventions (id INTEGER PRIMARY KEY, area TEXT NOT NULL, topic TEXT NOT NULL, rule TEXT NOT NULL, example TEXT, checked_by TEXT, reason TEXT, source TEXT);
  CREATE TABLE IF NOT EXISTS method_rules (id INTEGER PRIMARY KEY, area TEXT NOT NULL, rule TEXT NOT NULL, reason TEXT, since TEXT, source TEXT);
  CREATE TABLE IF NOT EXISTS preferences (id INTEGER PRIMARY KEY, topic TEXT NOT NULL, rule TEXT NOT NULL, why TEXT, how_to_apply TEXT, since TEXT, private INTEGER NOT NULL DEFAULT 1, memory TEXT);
  CREATE TABLE IF NOT EXISTS tools (name TEXT PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('binary', 'crate', 'hook', 'git-hook', 'script')), crate TEXT, run_by TEXT, does TEXT NOT NULL, replaces TEXT, source TEXT);
  CREATE VIRTUAL TABLE IF NOT EXISTS records_index USING fts5(
    ref UNINDEXED, kind UNINDEXED, id UNINDEXED, section UNINDEXED, text, tokenize = 'porter unicode61'
  );
  INSERT OR IGNORE INTO tables_registry (name, purpose) VALUES
    ('story_items', 'a story''s facts beyond its columns, one per row (story, n): its H2 section, the kind (paragraph, bullet, table row as ''header: cell · …'', code) and the item'),
    ('feature_items', 'a feature''s facts beyond its columns and the criteria and stories tables, one per row (feature, n)'),
    ('epic_items', 'an epic''s facts beyond its columns, one per row (epic, n)'),
    ('decision_items', 'an ADR''s facts beyond its columns, one per row (decision, n): mostly its Reasons and any other section'),
    ('retrospective_items', 'a retrospective''s facts, one per row (retrospective, n)'),
    ('rule_items', 'what a business-rule page (RULE-<page>-<section>) holds beyond the rows of rules, one per row (page, n)'),
    ('derivation_steps', 'a derivation''s steps, one per row (derivation, n): its section, the statement and the ids of the sources it cites'),
    ('note_findings', 'a research note''s, request''s or research page''s findings, one per row (note, n): its section, the finding and the ids of the sources it cites'),
    ('conventions', 'one convention of the code, the tests, the tooling or the writing, a row: its area, topic, rule, example, check, reason and source'),
    ('method_rules', 'the agent''s working method now, one rule a row: its area, the rule, its reason, since when, its source'),
    ('preferences', 'the Product Owner''s preferences and the project''s standing facts, one a row: topic, rule, why, how to apply, since, private, the memory it came from'),
    ('tools', 'one tool of the repository, a row: its name, kind, crate, who runs it, what it does, what it replaces, its source'),
    ('records_index', 'FTS5 (porter unicode61) over every text field of the agent''s tables, one row per source row: ref ''<table>:<key>'', kind (table), id (the record to show), section, text; kept by three triggers per table; query with MATCH and bm25(records_index)');
";

/// The columns schema 6 adds to a table of schema 4 or 5, each added only where the file lacks
/// it: the records restructured by hand before schema 6 already hold them.
const SCHEMA_6_COLUMNS: [(&str, &str, &str); 19] = [
    ("epics", "vision", "TEXT NOT NULL DEFAULT ''"),
    ("features", "hypothesis", "TEXT NOT NULL DEFAULT ''"),
    ("stories", "summary", "TEXT NOT NULL DEFAULT ''"),
    ("decisions", "decision", "TEXT"),
    ("decisions", "reason", "TEXT"),
    ("decisions", "replaces", "TEXT"),
    ("decisions", "replaced_by", "TEXT"),
    ("decisions", "removed_reason", "TEXT"),
    ("decisions", "changed", "TEXT"),
    ("decisions", "status_note", "TEXT NOT NULL DEFAULT ''"),
    ("decisions", "consequences", "TEXT NOT NULL DEFAULT ''"),
    ("decisions", "refs", "TEXT NOT NULL DEFAULT ''"),
    ("improvements", "status", "TEXT"),
    ("improvements", "closed", "TEXT"),
    ("improvements", "outcome", "TEXT"),
    ("improvements", "reviews", "TEXT"),
    ("incidents", "status", "TEXT"),
    ("incidents", "closed", "TEXT"),
    ("incidents", "outcome", "TEXT"),
];

/// A table the full-text index `records_index` reads, one index row per row of it: its key in
/// `ref` (`<table>:<key>`), the id `show` opens, the section and the columns joined into the text,
/// each expression written on the row `{r}`.
pub struct Indexed {
    /// The table.
    pub table: &'static str,
    /// The row's key, unique in the table.
    pub key: &'static str,
    /// The id of the record the row belongs to.
    pub id: &'static str,
    /// The section the row is under.
    pub section: &'static str,
    /// The columns of the text, joined by line feeds.
    pub columns: &'static [&'static str],
}

/// The tables `records_index` reads, as the records restructured before schema 6 index them.
pub const INDEXED: [Indexed; 28] = [
    indexed(
        "stories",
        "{r}.id",
        "{r}.id",
        "'story'",
        &["title", "summary", "criteria", "test_card", "question"],
    ),
    indexed(
        "features",
        "{r}.id",
        "{r}.id",
        "'feature'",
        &["title", "hypothesis"],
    ),
    indexed("epics", "{r}.id", "{r}.id", "'epic'", &["title", "vision"]),
    indexed("criteria", "{r}.id", "{r}.id", "'criterion'", &["text"]),
    indexed(
        "tasks",
        "{r}.story || '.T' || {r}.number",
        "{r}.story || '.T' || {r}.number",
        "'task'",
        &["title"],
    ),
    indexed(
        "decisions",
        "{r}.id",
        "{r}.id",
        "'decision'",
        &[
            "title",
            "principle",
            "decision",
            "reason",
            "consequences",
            "status_note",
        ],
    ),
    indexed("notes", "{r}.id", "{r}.id", "{r}.kind", &["title"]),
    indexed(
        "derivations",
        "{r}.id",
        "{r}.id",
        "'derivation'",
        &["title", "used_by"],
    ),
    indexed(
        "sources",
        "{r}.id",
        "{r}.id",
        "'source'",
        &["title", "used_for", "topic", "notes"],
    ),
    indexed(
        "rules",
        "{r}.id",
        "{r}.id",
        "{r}.section",
        &["name", "rule", "example"],
    ),
    indexed(
        "conventions",
        "{r}.id",
        "'conventions ' || {r}.id",
        "{r}.area || ' › ' || {r}.topic",
        &["rule", "example", "reason"],
    ),
    indexed(
        "method_rules",
        "{r}.id",
        "'method_rules ' || {r}.id",
        "{r}.area",
        &["rule", "reason"],
    ),
    indexed(
        "preferences",
        "{r}.id",
        "'preferences ' || {r}.id",
        "{r}.topic",
        &["rule", "how_to_apply"],
    ),
    indexed(
        "tools",
        "{r}.name",
        "{r}.name",
        "{r}.kind",
        &["does", "run_by", "replaces"],
    ),
    indexed(
        "incidents",
        "{r}.id",
        "'incidents ' || {r}.id",
        "{r}.tool",
        &["used", "failed", "instead"],
    ),
    indexed(
        "improvements",
        "{r}.id",
        "'improvements ' || {r}.id",
        "{r}.kind",
        &["item"],
    ),
    indexed(
        "feedback",
        "{r}.id",
        "'feedback ' || {r}.id",
        "'feedback'",
        &["happened", "should_have", "prevented_by"],
    ),
    indexed(
        "journal",
        "{r}.id",
        "'journal ' || {r}.id",
        "{r}.session",
        &["text"],
    ),
    indexed(
        "memories",
        "{r}.name",
        "'MEMORY-' || {r}.name",
        "{r}.type",
        &["description", "text"],
    ),
    indexed(
        "retrospectives",
        "{r}.id",
        "{r}.id",
        "'retrospective'",
        &["title"],
    ),
    indexed(
        "story_items",
        "{r}.story || ':' || {r}.n",
        "{r}.story",
        "{r}.section",
        &["item"],
    ),
    indexed(
        "feature_items",
        "{r}.feature || ':' || {r}.n",
        "{r}.feature",
        "{r}.section",
        &["item"],
    ),
    indexed(
        "epic_items",
        "{r}.epic || ':' || {r}.n",
        "{r}.epic",
        "{r}.section",
        &["item"],
    ),
    indexed(
        "decision_items",
        "{r}.decision || ':' || {r}.n",
        "{r}.decision",
        "{r}.section",
        &["item"],
    ),
    indexed(
        "retrospective_items",
        "{r}.retrospective || ':' || {r}.n",
        "{r}.retrospective",
        "{r}.section",
        &["item"],
    ),
    indexed(
        "rule_items",
        "{r}.page || ':' || {r}.n",
        "{r}.page",
        "{r}.section",
        &["item"],
    ),
    indexed(
        "derivation_steps",
        "{r}.derivation || ':' || {r}.n",
        "{r}.derivation",
        "{r}.section",
        &["statement"],
    ),
    indexed(
        "note_findings",
        "{r}.note || ':' || {r}.n",
        "{r}.note",
        "{r}.section",
        &["finding"],
    ),
];

/// An [`Indexed`] table, written on one line in [`INDEXED`].
const fn indexed(
    table: &'static str,
    key: &'static str,
    id: &'static str,
    section: &'static str,
    columns: &'static [&'static str],
) -> Indexed {
    Indexed {
        table,
        key,
        id,
        section,
        columns,
    }
}

/// The values of an index row of `table`, `(ref, kind, id, section, text)`, written on the row
/// named `row`: `new` in a trigger, the table's alias in a filling.
fn index_values(table: &Indexed, row: &str) -> String {
    let on_row = |expression: &str| expression.replace("{r}", row);
    let text: Vec<String> = table
        .columns
        .iter()
        .map(|column| format!("coalesce({row}.{column}, '')"))
        .collect();

    format!(
        "'{name}:' || {key}, '{name}', {id}, {section}, {text}",
        name = table.table,
        key = on_row(table.key),
        id = on_row(table.id),
        section = on_row(table.section),
        text = text.join(" || char(10) || ")
    )
}

/// The three triggers that keep `records_index` in step with a table, each made unless the file
/// holds it (`REF-SQLITE-FTS5`, §4.4.3, an external index kept by triggers).
#[must_use]
pub fn index_triggers(table: &Indexed) -> String {
    let name = table.table;
    let insert = format!(
        "INSERT INTO records_index (ref, kind, id, section, text) VALUES ({});",
        index_values(table, "new")
    );
    let delete = |row: &str| {
        format!(
            "DELETE FROM records_index WHERE ref = '{name}:' || {};",
            table.key.replace("{r}", row)
        )
    };

    format!(
        "CREATE TRIGGER IF NOT EXISTS records_index_{name}_insert AFTER INSERT ON {name} BEGIN {insert} END;
        CREATE TRIGGER IF NOT EXISTS records_index_{name}_delete AFTER DELETE ON {name} BEGIN {} END;
        CREATE TRIGGER IF NOT EXISTS records_index_{name}_update AFTER UPDATE ON {name} BEGIN {} {insert} END;",
        delete("old"),
        delete("old")
    )
}

/// Whether the file holds a table or a column of it.
fn holds(database: &Connection, table: &str, column: Option<&str>) -> rusqlite::Result<bool> {
    let query = match column {
        None => "SELECT count(*) FROM sqlite_master WHERE name = ?1",
        Some(_) => "SELECT count(*) FROM pragma_table_info(?1) WHERE name = ?2",
    };
    let count: i64 = match column {
        None => database.query_row(query, [table], |row| row.get(0))?,
        Some(column) => database.query_row(query, [table, column], |row| row.get(0))?,
    };

    Ok(count > 0)
}

/// Brings a file of schema 5 up to 6, additively: the item tables and the agent's tables made
/// where missing, each column of [`SCHEMA_6_COLUMNS`] added where missing, `records_index` made
/// and filled from every table it reads when the file lacks it, and its triggers made where
/// missing. The records restructured by hand before schema 6 hold it all: nothing is redone. A
/// `text` column of an older file stays, its rows kept, as no migration drops a column.
///
/// # Errors
/// When SQLite fails.
pub fn schema_6(database: &Connection) -> rusqlite::Result<()> {
    let had_index = holds(database, "records_index", None)?;

    database.execute_batch(SCHEMA_6_TABLES)?;
    for (table, column, declaration) in SCHEMA_6_COLUMNS {
        if !holds(database, table, Some(column))? {
            database.execute_batch(&format!(
                "ALTER TABLE {table} ADD COLUMN {column} {declaration}"
            ))?;
        }
    }
    for table in &INDEXED {
        if !had_index {
            database.execute_batch(&format!(
                "INSERT INTO records_index (ref, kind, id, section, text) SELECT {} FROM {} AS r",
                index_values(table, "r"),
                table.table
            ))?;
        }
        database.execute_batch(&index_triggers(table))?;
    }
    Ok(())
}

/// A failure of SQLite on the store's file, named by its path.
fn on_file(path: &Path, error: &impl std::fmt::Display) -> String {
    format!("{}: {error}", path.display())
}

/// Brings a file of an older schema up to `SCHEMA_VERSION`, in one `BEGIN IMMEDIATE`: of several
/// connections opening a new file at once, the first makes the tables while the others wait, then
/// find them made. Runs with the foreign keys off, as a rebuilt table needs
/// (`REF-SQLITE-ALTER-TABLE`, §8); each child row is copied joined to its document, so that none
/// can cite a document the file lacks.
fn migrate(database: &Connection) -> rusqlite::Result<()> {
    if schema_version(database)? >= SCHEMA_VERSION {
        return Ok(());
    }

    let transaction = Transaction::new_unchecked(database, TransactionBehavior::Immediate)?;
    let version = schema_version(&transaction)?;

    for migration in MIGRATIONS
        .iter()
        .skip(usize::try_from(version).unwrap_or(0))
    {
        transaction.execute_batch(migration)?;
    }
    if version < 5 {
        make_library_tables(&transaction)?;
    }
    if version < 6 {
        schema_6(&transaction)?;
    }
    if version < SCHEMA_VERSION {
        transaction.pragma_update(None, "user_version", SCHEMA_VERSION)?;
    }
    transaction.commit()
}

/// Whether the file is in WAL, as this connection reads its mode.
fn is_wal(database: &Connection) -> rusqlite::Result<bool> {
    let mode: String = database.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;

    Ok(mode.eq_ignore_ascii_case("wal"))
}

/// Switches the file to WAL; the mode the switch leaves it in, `wal`, or the mode it kept when
/// refused (`REF-SQLITE-PRAGMA`, `journal_mode`).
fn switch_to_wal(database: &Connection) -> rusqlite::Result<String> {
    database.query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))
}

/// Puts the file in WAL. The switch needs the file alone, and SQLite refused it at once, without
/// waiting, while another connection wrote, as EN-152 measured (`the records' conventions, area records`, Two
/// worktrees at once), the mode it kept returned (`REF-SQLITE-PRAGMA`): that write is waited for,
/// by a `BEGIN IMMEDIATE` within the busy timeout, and the mode read again, the other connection
/// having maybe switched the file itself, before one more switch; that one failing, or leaving the
/// file in another mode, fails the opening.
fn in_wal(database: &Connection) -> rusqlite::Result<()> {
    if is_wal(database)?
        || switch_to_wal(database).is_ok_and(|mode| mode.eq_ignore_ascii_case("wal"))
    {
        return Ok(());
    }
    database.execute_batch("BEGIN IMMEDIATE; ROLLBACK;")?;
    if is_wal(database)? {
        return Ok(());
    }

    let mode = switch_to_wal(database)?;

    if mode.eq_ignore_ascii_case("wal") {
        Ok(())
    } else {
        Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_ERROR),
            Some(format!(
                "the file stays in the {mode} journal mode, WAL refused"
            )),
        ))
    }
}

/// Sets what a connection needs: WAL, set once on the file and kept by it (`REF-SQLITE-WAL`, §3.3),
/// the migration, then the deletes cascading to a document's rows, which SQLite turns on per
/// connection, outside any transaction.
fn prepare(database: &Connection) -> rusqlite::Result<()> {
    database.pragma_update(None, "foreign_keys", false)?;
    in_wal(database)?;
    migrate(database)?;
    database.pragma_update(None, "foreign_keys", true)
}

/// Opens the store, creating its folder and its file when missing, in WAL, and bringing a file of
/// an older schema up to `SCHEMA_VERSION` in one `BEGIN IMMEDIATE` transaction; a file of a newer
/// schema is opened as it is, for reading.
///
/// # Errors
/// When the folder cannot be made, or SQLite cannot open the file, put it in WAL or create the
/// tables.
pub fn open_store(path: &Path) -> Result<Connection, String> {
    if let Some(folder) = path
        .parent()
        .filter(|folder| !folder.as_os_str().is_empty())
    {
        fs::create_dir_all(folder).map_err(|error| on_file(folder, &error))?;
    }

    let database = Connection::open(path).map_err(|error| on_file(path, &error))?;

    prepare(&database).map_err(|error| on_file(path, &error))?;
    Ok(database)
}

/// The schema's number the file holds, `PRAGMA user_version`.
///
/// # Errors
/// When SQLite fails to read.
pub fn schema_version(database: &Connection) -> rusqlite::Result<i64> {
    database.query_row("PRAGMA user_version", [], |row| row.get(0))
}

/// Runs a write in one `BEGIN IMMEDIATE` transaction, committed when the work succeeds and rolled
/// back otherwise; refused, untouched, on a file whose schema is newer than this binary's, read
/// once the lock is held so that no newer binary migrates the file in between.
///
/// # Errors
/// When the file's schema is newer, or SQLite fails, a busy timeout spent included.
pub fn write<T>(
    database: &Connection,
    work: impl FnOnce(&Transaction<'_>) -> rusqlite::Result<T>,
) -> Result<T, String> {
    write_or_refuse(database, |transaction| {
        work(transaction).map_err(|error| error.to_string())
    })
}

/// [`write`] of a work that may refuse with a reason of its own, rolled back as a failure of
/// SQLite is (EN-154).
///
/// # Errors
/// As [`write`], and the work's own reason.
pub fn write_or_refuse<T>(
    database: &Connection,
    work: impl FnOnce(&Transaction<'_>) -> Result<T, String>,
) -> Result<T, String> {
    let failed = |error: rusqlite::Error| error.to_string();
    let transaction =
        Transaction::new_unchecked(database, TransactionBehavior::Immediate).map_err(failed)?;
    let version = schema_version(&transaction).map_err(failed)?;

    if version > SCHEMA_VERSION {
        return Err(format!(
            "the file's schema is {version}, newer than this binary's {SCHEMA_VERSION}: rebuild the tools from your base, target/release/git-sequences sync"
        ));
    }

    let done = work(&transaction)?;

    transaction.commit().map_err(failed)?;
    Ok(done)
}
