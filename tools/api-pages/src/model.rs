//! The library's API as plain data: read from the sources by `extract`, written as pages by
//! `page`, checked by `checks`.

/// What an export is: it names the page's folder, its title and its group in the index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    /// `export type`.
    TypeAlias,
    /// `export const`.
    Variable,
    /// `export function`.
    Function,
}

impl Kind {
    /// The three kinds, in the order of the index and of the sidebar.
    pub const ALL: [Kind; 3] = [Kind::TypeAlias, Kind::Variable, Kind::Function];

    /// The folder of its pages, next to the index.
    #[must_use]
    pub fn folder(self) -> &'static str {
        match self {
            Kind::TypeAlias => "type-aliases",
            Kind::Variable => "variables",
            Kind::Function => "functions",
        }
    }

    /// The word before its name in a page's title.
    #[must_use]
    pub fn title(self) -> &'static str {
        match self {
            Kind::TypeAlias => "Type Alias",
            Kind::Variable => "Variable",
            Kind::Function => "Function",
        }
    }

    /// Its heading in the index and its group in the sidebar.
    #[must_use]
    pub fn group(self) -> &'static str {
        match self {
            Kind::TypeAlias => "Type Aliases",
            Kind::Variable => "Variables",
            Kind::Function => "Functions",
        }
    }
}

/// What a name in a type refers to, as the module's scopes resolve it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// Declared or imported in the module: an export of the library, or an internal name.
    Module,
    /// A type parameter of the declaration.
    TypeParameter,
    /// Declared nowhere in the module: a global of the language (`Readonly`, `Uint8Array`).
    Global,
}

/// A type as written in an annotation.
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// A keyword: `number`, `string`, `boolean`…
    Intrinsic(String),
    /// A literal type, as the compiler prints it: `"min"`, `360`, `true`.
    Literal(String),
    /// A named type, with its arguments.
    Reference {
        /// Its name, dotted when qualified.
        name: String,
        /// What it refers to.
        target: Target,
        /// Its type arguments.
        arguments: Vec<Type>,
    },
    /// `typeof name`.
    Query {
        /// The value's name.
        name: String,
        /// What it refers to.
        target: Target,
    },
    /// `T[]`.
    Array(Box<Type>),
    /// `[A, B]`.
    Tuple(Vec<Type>),
    /// `A | B`.
    Union(Vec<Type>),
    /// `A & B`.
    Intersection(Vec<Type>),
    /// `{ … }`, its members.
    Object(Vec<Member>),
    /// `(a: A) => R`.
    Function(Box<Signature>),
    /// `T[K]`.
    IndexedAccess(Box<Type>, Box<Type>),
    /// `readonly T`, `keyof T`, `unique T`.
    Operator(&'static str, Box<Type>),
}

/// A property of an object type.
#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    /// Its name.
    pub name: String,
    /// Whether it is `readonly`.
    pub readonly: bool,
    /// Whether it is optional (`?`).
    pub optional: bool,
    /// Its type.
    pub value: Type,
    /// Its `TSDoc` block.
    pub doc: Option<Doc>,
    /// Its line in its file, from 1.
    pub line: usize,
}

/// A parameter of a function.
#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    /// Its name.
    pub name: String,
    /// Its type.
    pub value: Type,
    /// Whether it is optional (`?`).
    pub optional: bool,
    /// Whether it is a rest parameter (`...`).
    pub rest: bool,
}

/// A type parameter, with its constraint.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeParameter {
    /// Its name.
    pub name: String,
    /// The type after `extends`.
    pub constraint: Option<Type>,
}

/// A function's type parameters, parameters and return type.
#[derive(Clone, Debug, PartialEq)]
pub struct Signature {
    /// Its type parameters.
    pub type_parameters: Vec<TypeParameter>,
    /// Its parameters.
    pub parameters: Vec<Parameter>,
    /// Its return type.
    pub returns: Type,
}

/// What an export declares.
#[derive(Clone, Debug, PartialEq)]
pub enum Body {
    /// A function.
    Function(Signature),
    /// A type alias.
    TypeAlias {
        /// Its type parameters.
        type_parameters: Vec<TypeParameter>,
        /// The aliased type.
        value: Type,
    },
    /// A constant.
    Variable {
        /// Its type, written or read from a literal value.
        value: Type,
        /// Its value as written, shown when its type is that literal.
        literal: Option<String>,
    },
}

/// A `TSDoc` block: its summary and its block tags, in order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Doc {
    /// The text before the first tag.
    pub summary: String,
    /// The block tags.
    pub tags: Vec<Tag>,
}

/// A block tag: `@param name - text`, `@returns text`, `@kind math`…
#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    /// Its name, without `@`.
    pub name: String,
    /// The name it documents, for `@param` and `@template`.
    pub target: Option<String>,
    /// Its text.
    pub text: String,
}

impl Doc {
    /// The text of the tag documenting a name (`@param`, `@template`).
    #[must_use]
    pub fn named(&self, tag: &str, target: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|each| each.name == tag && each.target.as_deref() == Some(target))
            .map(|each| each.text.as_str())
    }

    /// The text of the first tag of a name (`@returns`).
    #[must_use]
    pub fn first(&self, tag: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|each| each.name == tag)
            .map(|each| each.text.as_str())
    }
}

/// One export of the library.
#[derive(Clone, Debug, PartialEq)]
pub struct Export {
    /// Its name.
    pub name: String,
    /// Its kind.
    pub kind: Kind,
    /// Its file, relative to the entry's folder, with `/`.
    pub file: String,
    /// Its line, from 1.
    pub line: usize,
    /// Its `TSDoc` block.
    pub doc: Option<Doc>,
    /// What it declares.
    pub body: Body,
}
