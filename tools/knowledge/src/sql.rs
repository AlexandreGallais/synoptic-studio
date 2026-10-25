//! `knowledge sql "<statement>"` (EN-154): one statement of SQL on the records, for the agent's own
//! questions and its own tables. A reading prints its rows; a write runs in `BEGIN IMMEDIATE` and
//! may create a table or an index, insert, update or delete, never drop nor rewrite a core table
//! nor write a document's text, which `sync` keeps with its index; a document's row may be
//! deleted (its fields, sections and citations cascade, its index kept by its trigger) or its
//! columns updated. A row of a table `records_index` reads is indexed by that table's triggers.
//! What a statement does is read from SQLite itself, by the authorizer it calls for each action while it compiles the
//! statement (`REF-SQLITE-AUTHORIZER`), kept in place while the statement runs since a change of
//! the schema compiles it again; never from its text but for `ALTER TABLE`, whose kind of change
//! the authorizer does not say. A new table is registered in `tables_registry` with its purpose,
//! and a row written without a branch takes the branch checked out.

use std::sync::{Arc, Mutex, PoisonError};

use knowledge_call::escaped;
pub use knowledge_call::unescaped;
use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Statement, params};

use crate::store::{CORE_TABLES, WRITTEN_BY_THE_TOOL};

/// What a statement asked for, as the authorizer saw it: the tables it creates and drops, and
/// the first action refused, with its reason.
#[derive(Clone, Debug, Default)]
struct Seen {
    created: Vec<String>,
    dropped: Vec<String>,
    refusal: Option<String>,
    /// Whether the statement deletes or updates a document's row: the writes that follow are its
    /// cascades to its fields, sections and citations and its index's trigger.
    cascades: bool,
}

/// Whether an action writes one of a document's own tables, as the cascades of a document's row
/// deleted or updated do: its fields, sections, citations and their index.
fn is_cascade(action: &AuthAction<'_>) -> bool {
    match *action {
        AuthAction::Insert { table_name }
        | AuthAction::Update { table_name, .. }
        | AuthAction::Delete { table_name } => {
            ["fields", "sections", "links", "sections_index"].contains(&table_name)
                || table_name.starts_with("sections_index_")
        }
        _ => false,
    }
}

/// Whether a table is one of the tables FTS5 keeps for a full-text index of the tool: the
/// knowledge's `sections_index_…` or the library's `chunks_…`.
fn is_index_shadow(table: &str) -> bool {
    table.starts_with("sections_index_") || table.starts_with("chunks_")
}

/// Whether a table is one the tool keeps, its full-text indexes' own tables among them; those of
/// `records_index` are written by the triggers of every table it reads, so a write to them is no
/// refusal.
#[must_use]
pub fn is_core(table: &str) -> bool {
    CORE_TABLES.contains(&table) || is_index_shadow(table) || table.starts_with("records_index_")
}

/// Whether a table is written by the tool alone.
fn is_written_by_the_tool(table: &str) -> bool {
    WRITTEN_BY_THE_TOOL.contains(&table) || is_index_shadow(table)
}

/// Whether a statement's text is an `ALTER TABLE <name> ADD …`, the one change of a table that
/// adds: its words read in capitals, whatever the spaces.
#[must_use]
pub fn is_added_column(statement: &str) -> bool {
    let words: Vec<String> = statement
        .split_whitespace()
        .map(str::to_ascii_uppercase)
        .collect();

    matches!(
        words.as_slice(),
        [alter, table, _, add, ..] if alter == "ALTER" && table == "TABLE" && add == "ADD"
    )
}

/// Why an action is refused, `None` when it may run.
#[must_use]
pub fn refusal(action: &AuthAction<'_>, statement: &str) -> Option<String> {
    let kept = |what: &str, table: &str| {
        Some(format!(
            "{what} {table}: a core table of the records, kept by the tool"
        ))
    };

    match *action {
        AuthAction::DropTable { table_name } | AuthAction::DropVtable { table_name, .. }
            if is_core(table_name) =>
        {
            kept("DROP TABLE", table_name)
        }
        AuthAction::DropIndex { table_name, .. } | AuthAction::DropTrigger { table_name, .. }
            if is_core(table_name) =>
        {
            kept("DROP of an index or a trigger of", table_name)
        }
        AuthAction::AlterTable { table_name, .. }
            if is_core(table_name) && !is_added_column(statement) =>
        {
            kept("ALTER TABLE other than ADD on", table_name)
        }
        AuthAction::CreateTrigger { table_name, .. } if is_written_by_the_tool(table_name) => {
            kept("CREATE TRIGGER on", table_name)
        }
        AuthAction::Update { table_name, .. } | AuthAction::Delete { table_name }
            if table_name == "documents" =>
        {
            None
        }
        AuthAction::Insert { table_name }
        | AuthAction::Update { table_name, .. }
        | AuthAction::Delete { table_name }
            if is_written_by_the_tool(table_name) =>
        {
            Some(format!(
                "{table_name} is written by the tool alone, so that its full-text index stays whole"
            ))
        }
        AuthAction::Pragma {
            pragma_name,
            pragma_value: Some(_),
        } => Some(format!(
            "PRAGMA {pragma_name} with a value: the tool sets the file's pragmas"
        )),
        AuthAction::Attach { .. } | AuthAction::Detach { .. } => {
            Some("ATTACH and DETACH: the records are one file".to_owned())
        }
        AuthAction::Transaction { .. } | AuthAction::Savepoint { .. } => {
            Some("a transaction: each write runs in its own BEGIN IMMEDIATE".to_owned())
        }
        _ => None,
    }
}

/// A value as `sql` prints it: text and numbers as they are, a null empty, a blob by its size;
/// a backslash, a tab and a line end escaped by `knowledge_call`, which reads them back, so that a
/// row stays one line of cells.
fn cell(value: ValueRef<'_>) -> String {
    let text = match value {
        ValueRef::Null => String::new(),
        ValueRef::Integer(number) => number.to_string(),
        ValueRef::Real(number) => number.to_string(),
        ValueRef::Text(bytes) => String::from_utf8_lossy(bytes).into_owned(),
        ValueRef::Blob(bytes) => format!("<{} bytes>", bytes.len()),
    };

    escaped(&text)
}

/// Notes an action the authorizer allows: a table created or dropped, a document's row deleted or
/// updated.
fn note(seen: &mut Seen, action: &AuthAction<'_>) {
    match *action {
        AuthAction::Update { table_name, .. } | AuthAction::Delete { table_name }
            if table_name == "documents" =>
        {
            seen.cascades = true;
        }
        AuthAction::CreateTable { table_name } | AuthAction::CreateVtable { table_name, .. } => {
            seen.created.push(table_name.to_owned());
        }
        AuthAction::DropTable { table_name } | AuthAction::DropVtable { table_name, .. } => {
            seen.dropped.push(table_name.to_owned());
        }
        _ => {}
    }
}

/// Compiles a statement with the authorizer watching and runs `work` on it, the authorizer still
/// in place, so that what it asks is known before it runs and a compilation again during its run
/// is watched too (`REF-SQLITE-AUTHORIZER`, Limitations); the authorizer removed after, whatever
/// the result.
fn watched<T>(
    database: &Connection,
    statement: &str,
    work: impl FnOnce(&mut Statement<'_>, &Seen) -> Result<T, String>,
) -> Result<(T, Seen), String> {
    let seen = Arc::new(Mutex::new(Seen::default()));
    let watcher = Arc::clone(&seen);
    let text = statement.to_owned();
    let failed = |error: rusqlite::Error| error.to_string();

    database
        .authorizer(Some(move |context: AuthContext<'_>| {
            let mut seen = watcher.lock().unwrap_or_else(PoisonError::into_inner);

            if let Some(reason) = refusal(&context.action, &text)
                && !(seen.cascades && is_cascade(&context.action))
            {
                seen.refusal.get_or_insert(reason);
                return Authorization::Deny;
            }
            note(&mut seen, &context.action);
            Authorization::Allow
        }))
        .map_err(failed)?;

    let snapshot = || seen.lock().unwrap_or_else(PoisonError::into_inner).clone();
    let done = match database.prepare(statement) {
        Ok(mut prepared) => {
            let before = snapshot();

            match &before.refusal {
                Some(reason) => Err(format!("refused: {reason}")),
                None => work(&mut prepared, &before),
            }
        }
        Err(error) => Err(snapshot()
            .refusal
            .map_or_else(|| error.to_string(), |reason| format!("refused: {reason}"))),
    };
    let none: Option<fn(AuthContext<'_>) -> Authorization> = None;

    database.authorizer(none).map_err(failed)?;
    done.map(|value| (value, snapshot()))
}

/// The rows of a reading, its column names first, each row a line of cells joined by tabs.
///
/// # Errors
/// When the statement is refused, is not a reading, or SQLite fails.
pub fn read(database: &Connection, statement: &str) -> Result<String, String> {
    let (text, _) = watched(database, statement, |prepared, _| {
        if !prepared.readonly() {
            return Err("not a reading".to_owned());
        }

        let count = prepared.column_count();
        let mut text = prepared.column_names().join("\t") + "\n";
        let mut rows = prepared.query([]).map_err(|error| error.to_string())?;

        while let Some(row) = rows.next().map_err(|error| error.to_string())? {
            let cells: Vec<String> = (0..count)
                .map(|at| row.get_ref(at).map(cell).unwrap_or_default())
                .collect();

            text.push_str(&cells.join("\t"));
            text.push('\n');
        }
        Ok(text)
    })?;

    Ok(text)
}

/// Whether a statement only reads, as SQLite compiles it, the authorizer refusing what `sql` never
/// runs.
///
/// # Errors
/// When the statement is refused or does not compile.
pub fn is_reading(database: &Connection, statement: &str) -> Result<bool, String> {
    watched(database, statement, |prepared, _| Ok(prepared.readonly())).map(|(reading, _)| reading)
}

/// The tables of the file that hold both a `branch` and a `state` column but the documents: the
/// backlog's and the agent's own, whose rows follow their branch.
///
/// # Errors
/// When SQLite fails.
pub fn branch_tables(database: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut statement = database.prepare(
        "SELECT name FROM sqlite_master AS tables WHERE type = 'table' AND name NOT LIKE 'sqlite_%'
          AND name <> 'documents'
          AND EXISTS (SELECT 1 FROM pragma_table_info(tables.name) WHERE name = 'branch')
          AND EXISTS (SELECT 1 FROM pragma_table_info(tables.name) WHERE name = 'state')
        ORDER BY name",
    )?;

    statement
        .query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<_>>()
}

/// A table's name between double quotes, for a statement built around it.
#[must_use]
pub fn quoted(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// The tables a statement created, but SQLite's own.
fn created(seen: &Seen) -> Vec<&str> {
    seen.created
        .iter()
        .map(String::as_str)
        .filter(|table| !table.starts_with("sqlite_"))
        .collect()
}

/// Runs a write in the transaction given: the statement, then each table it created registered
/// with its purpose and each one it dropped struck from the registry, then the rows written
/// without a branch given the branch checked out. What it prints: the rows changed.
///
/// # Errors
/// When the statement is refused, creates a table without a purpose, or SQLite fails.
pub fn write(
    database: &Connection,
    statement: &str,
    purpose: Option<&str>,
    branch: &str,
) -> Result<String, String> {
    let failed = |error: rusqlite::Error| error.to_string();
    let purpose = purpose.map(str::trim).filter(|purpose| !purpose.is_empty());
    let (changed, seen) = watched(database, statement, |prepared, seen| {
        let tables = created(seen);

        if !tables.is_empty() && purpose.is_none() {
            return Err(format!(
                "CREATE TABLE {}: give its purpose with --purpose \"<what one row holds>\", for tables_registry",
                tables.join(", ")
            ));
        }
        // The rows the statement changed: `execute`'s count, which leaves out the rows its
        // triggers write (the full-text index's), but stays the last change's for a statement
        // that changes no row, as one creating a table: none then, by the connection's total.
        let before = database.total_changes();
        let counted = prepared.execute([]).map_err(failed)?;

        Ok(if database.total_changes() == before {
            0
        } else {
            u64::try_from(counted).unwrap_or(0)
        })
    })?;
    let tables = created(&seen);

    for table in &tables {
        database
            .execute(
                "INSERT INTO tables_registry (name, purpose) VALUES (?, ?)
                ON CONFLICT (name) DO UPDATE SET purpose = excluded.purpose",
                params![table, purpose.unwrap_or_default()],
            )
            .map_err(failed)?;
    }
    for table in &seen.dropped {
        database
            .execute("DELETE FROM tables_registry WHERE name = ?", [table])
            .map_err(failed)?;
    }
    if !branch.is_empty() {
        for table in branch_tables(database).map_err(failed)? {
            database
                .execute(
                    &format!("UPDATE {} SET branch = ? WHERE branch = ''", quoted(&table)),
                    [branch],
                )
                .map_err(failed)?;
        }
    }

    let registered = if tables.is_empty() {
        String::new()
    } else {
        format!(", {} registered", tables.join(", "))
    };

    Ok(format!(
        "{changed} {} changed{registered}\n",
        if changed == 1 { "row" } else { "rows" }
    ))
}
