//! A small backtracking matcher for the ECMAScript regular expressions Unicorn's comment and URL
//! rules carry (EN-080), so that each is written as the plugin writes it: literals, classes,
//! `\d \w \s \b`, `\p{…}` by its common properties, groups, alternation, greedy and lazy
//! quantifiers, anchors, lookahead and lookbehind, with the `i` and `m` flags (ECMA-262,
//! Pattern Semantics, read for its backtracking order). The text is read by code point.

/// A class of characters: its items, and whether it is negated.
#[derive(Clone, Debug)]
struct Class {
    /// Whether it matches what its items do not.
    negated: bool,
    /// Its items.
    items: Vec<ClassItem>,
}

/// One item of a class.
#[derive(Clone, Debug)]
enum ClassItem {
    /// A character.
    Char(char),
    /// A range of characters.
    Range(char, char),
    /// A named set: `\d`, `\w`, `\s`, a property, or their negation.
    Set(Set, bool),
}

/// A named set of characters.
#[derive(Clone, Copy, Debug)]
enum Set {
    /// `\d`.
    Digit,
    /// `\w`.
    Word,
    /// `\s`.
    Space,
    /// `\p{Letter}`.
    Letter,
    /// `\p{Lowercase_Letter}`.
    Lowercase,
    /// `\p{Uppercase_Letter}`.
    Uppercase,
    /// `\p{Mark}`.
    Mark,
    /// `\p{Number}`.
    Number,
}

/// A node of a pattern.
#[derive(Clone, Debug)]
enum Node {
    /// A character.
    Char(char),
    /// `.`: any character but a line terminator.
    Any,
    /// A class.
    Class(Class),
    /// `^`.
    Start,
    /// `$`.
    End,
    /// `\b` (true) or `\B` (false).
    Boundary(bool),
    /// A group, captured at an index or not.
    Group(Box<Node>, Option<usize>),
    /// A sequence.
    Concat(Vec<Node>),
    /// An alternation.
    Alt(Vec<Node>),
    /// A quantified node: minimum, maximum, greedy.
    Repeat(Box<Node>, usize, Option<usize>, bool),
    /// A lookaround: ahead or behind, negated or not.
    Look(Box<Node>, bool, bool),
}

/// A compiled pattern with its flags.
#[derive(Clone, Debug)]
pub struct Pattern {
    /// The root node.
    root: Node,
    /// The `i` flag.
    ignore_case: bool,
    /// The `m` flag.
    multiline: bool,
    /// The number of capturing groups.
    groups: usize,
    /// The names of the named groups, by index.
    names: Vec<(String, usize)>,
    /// The characters a match can start with, when they are known: a match is tried only there.
    first: Option<Vec<char>>,
}

/// A match: its range in code points and its groups' ranges.
#[derive(Clone, Debug)]
pub struct Match {
    /// Where it starts.
    pub start: usize,
    /// Where it ends.
    pub end: usize,
    /// Each group's range, when it took part.
    pub groups: Vec<Option<(usize, usize)>>,
}

/// The parser's state over the pattern's characters.
struct Parser<'p> {
    /// The pattern's characters.
    chars: &'p [char],
    /// The next one.
    at: usize,
    /// The capturing groups seen.
    groups: usize,
    /// The named groups seen.
    names: Vec<(String, usize)>,
}

impl Parser<'_> {
    /// The next character, read.
    fn next(&mut self) -> Option<char> {
        let character = self.chars.get(self.at).copied();
        self.at += 1;
        character
    }

    /// The next character, not read.
    fn peek(&self) -> Option<char> {
        self.chars.get(self.at).copied()
    }

    /// Whether the text ahead starts with a string, read if so.
    fn eat(&mut self, text: &str) -> bool {
        let wanted: Vec<char> = text.chars().collect();
        if self.chars.get(self.at..self.at + wanted.len()) == Some(&wanted[..]) {
            self.at += wanted.len();
            true
        } else {
            false
        }
    }

    /// An alternation.
    fn alternation(&mut self) -> Node {
        let mut branches = vec![self.sequence()];
        while self.peek() == Some('|') {
            self.at += 1;
            branches.push(self.sequence());
        }
        if branches.len() == 1 {
            branches.remove(0)
        } else {
            Node::Alt(branches)
        }
    }

    /// A sequence of quantified atoms.
    fn sequence(&mut self) -> Node {
        let mut nodes = Vec::new();
        while let Some(character) = self.peek() {
            if character == '|' || character == ')' {
                break;
            }
            let atom = self.atom();
            nodes.push(self.quantified(atom));
        }
        Node::Concat(nodes)
    }

    /// An atom with its quantifier, when one follows.
    fn quantified(&mut self, atom: Node) -> Node {
        let (min, max) = match self.peek() {
            Some('?') => (0, Some(1)),
            Some('*') => (0, None),
            Some('+') => (1, None),
            Some('{') => {
                let start = self.at;
                self.at += 1;
                let mut digits = String::new();
                while let Some(each) = self.peek().filter(char::is_ascii_digit) {
                    digits.push(each);
                    self.at += 1;
                }
                let min = digits.parse().ok();
                let max = if self.eat(",") {
                    let mut more = String::new();
                    while let Some(each) = self.peek().filter(char::is_ascii_digit) {
                        more.push(each);
                        self.at += 1;
                    }
                    if more.is_empty() {
                        None
                    } else {
                        more.parse().ok()
                    }
                } else {
                    min
                };
                if let (Some(min), true) = (min, self.eat("}")) {
                    let greedy = !self.eat("?");
                    return Node::Repeat(Box::new(atom), min, max, greedy);
                }
                self.at = start;
                return atom;
            }
            _ => return atom,
        };
        self.at += 1;
        let greedy = !self.eat("?");
        Node::Repeat(Box::new(atom), min, max, greedy)
    }

    /// One atom.
    fn atom(&mut self) -> Node {
        let Some(character) = self.next() else {
            return Node::Concat(Vec::new());
        };
        match character {
            '^' => Node::Start,
            '$' => Node::End,
            '.' => Node::Any,
            '[' => Node::Class(self.class()),
            '(' => self.group(),
            '\\' => self.escape(false),
            other => Node::Char(other),
        }
    }

    /// A group, a lookaround, or a named group.
    fn group(&mut self) -> Node {
        let (look, capture) = if self.eat("?:") {
            (None, false)
        } else if self.eat("?=") {
            (Some((true, false)), false)
        } else if self.eat("?!") {
            (Some((true, true)), false)
        } else if self.eat("?<=") {
            (Some((false, false)), false)
        } else if self.eat("?<!") {
            (Some((false, true)), false)
        } else if self.eat("?<") {
            let mut name = String::new();
            while let Some(each) = self.next() {
                if each == '>' {
                    break;
                }
                name.push(each);
            }
            self.names.push((name, self.groups));
            (None, true)
        } else {
            (None, true)
        };
        let index = capture.then(|| {
            self.groups += 1;
            self.groups - 1
        });
        let inner = self.alternation();
        self.eat(")");
        match look {
            Some((ahead, negated)) => Node::Look(Box::new(inner), ahead, negated),
            None => Node::Group(Box::new(inner), index),
        }
    }

    /// The code point of `\u{…}` or `\uHHHH`, after the `u`.
    fn unicode(&mut self) -> char {
        let mut digits = String::new();
        if self.eat("{") {
            while let Some(each) = self.next() {
                if each == '}' {
                    break;
                }
                digits.push(each);
            }
        } else {
            for _ in 0..4 {
                digits.extend(self.next());
            }
        }
        u32::from_str_radix(&digits, 16)
            .ok()
            .and_then(char::from_u32)
            .unwrap_or('\u{FFFD}')
    }

    /// A property of `\p{…}`.
    fn property(&mut self) -> Set {
        let mut name = String::new();
        if self.eat("{") {
            while let Some(each) = self.next() {
                if each == '}' {
                    break;
                }
                name.push(each);
            }
        }
        match name.as_str() {
            "Lowercase_Letter" | "Ll" => Set::Lowercase,
            "Uppercase_Letter" | "Lu" => Set::Uppercase,
            "Mark" | "M" => Set::Mark,
            "Number" | "N" => Set::Number,
            _ => Set::Letter,
        }
    }

    /// An escape, in a class or not.
    fn escape(&mut self, in_class: bool) -> Node {
        let Some(character) = self.next() else {
            return Node::Char('\\');
        };
        let set = |set: Set, negated: bool| {
            Node::Class(Class {
                negated: false,
                items: vec![ClassItem::Set(set, negated)],
            })
        };
        match character {
            'd' => set(Set::Digit, false),
            'D' => set(Set::Digit, true),
            'w' => set(Set::Word, false),
            'W' => set(Set::Word, true),
            's' => set(Set::Space, false),
            'S' => set(Set::Space, true),
            'p' => {
                let property = self.property();
                set(property, false)
            }
            'P' => {
                let property = self.property();
                set(property, true)
            }
            'b' if !in_class => Node::Boundary(true),
            'B' if !in_class => Node::Boundary(false),
            'b' => Node::Char('\u{8}'),
            'n' => Node::Char('\n'),
            'r' => Node::Char('\r'),
            't' => Node::Char('\t'),
            'f' => Node::Char('\u{C}'),
            'v' => Node::Char('\u{B}'),
            '0' => Node::Char('\0'),
            'u' => Node::Char(self.unicode()),
            'x' => {
                let digits: String = [self.next(), self.next()].into_iter().flatten().collect();
                Node::Char(
                    u32::from_str_radix(&digits, 16)
                        .ok()
                        .and_then(char::from_u32)
                        .unwrap_or('x'),
                )
            }
            other => Node::Char(other),
        }
    }

    /// A class, after its `[`.
    fn class(&mut self) -> Class {
        let negated = self.eat("^");
        let mut items = Vec::new();
        while let Some(character) = self.next() {
            if character == ']' {
                break;
            }
            let item = if character == '\\' {
                match self.escape(true) {
                    Node::Char(each) => ClassItem::Char(each),
                    Node::Class(class) => class
                        .items
                        .into_iter()
                        .next()
                        .unwrap_or(ClassItem::Char('\\')),
                    _ => ClassItem::Char('\\'),
                }
            } else {
                ClassItem::Char(character)
            };
            if let ClassItem::Char(low) = item
                && self.peek() == Some('-')
                && self.chars.get(self.at + 1).is_some_and(|next| *next != ']')
            {
                self.at += 1;
                let high = match self.next() {
                    Some('\\') => match self.escape(true) {
                        Node::Char(each) => each,
                        _ => low,
                    },
                    Some(each) => each,
                    None => low,
                };
                items.push(ClassItem::Range(low, high));
                continue;
            }
            items.push(item);
        }
        Class { negated, items }
    }
}

/// Whether a character is a combining mark, by the common blocks of marks.
fn is_mark(character: char) -> bool {
    matches!(u32::from(character),
        0x0300..=0x036F | 0x0483..=0x0489 | 0x0591..=0x05BD | 0x0610..=0x061A | 0x064B..=0x065F
        | 0x0900..=0x0903 | 0x093A..=0x094F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF
        | 0x20D0..=0x20FF | 0xFE00..=0xFE0F | 0xFE20..=0xFE2F)
}

/// Whether a character is a word character, `\w`.
#[must_use]
pub fn is_word(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

/// Whether a character is whitespace as `\s` reads it.
#[must_use]
pub fn is_space(character: char) -> bool {
    matches!(
        character,
        ' ' | '\t' | '\n' | '\u{B}' | '\u{C}' | '\r' | '\u{A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
                | '\u{FEFF}'
    )
}

/// Whether a character belongs to a named set.
fn in_set(set: Set, character: char) -> bool {
    match set {
        Set::Digit => character.is_ascii_digit(),
        Set::Word => is_word(character),
        Set::Space => is_space(character),
        Set::Letter => character.is_alphabetic() && !is_mark(character),
        Set::Lowercase => character.is_lowercase() && character.is_alphabetic(),
        Set::Uppercase => character.is_uppercase(),
        Set::Mark => is_mark(character),
        Set::Number => character.is_numeric(),
    }
}

/// A character folded as the `i` flag compares it.
fn fold(character: char) -> char {
    match character {
        '\u{17F}' => 's',
        '\u{212A}' => 'k',
        other => other.to_lowercase().next().unwrap_or(other),
    }
}

impl Class {
    /// Whether the class holds a character.
    fn holds(&self, character: char, ignore_case: bool) -> bool {
        let candidates: Vec<char> = if ignore_case {
            let mut each = vec![character, fold(character)];
            each.extend(character.to_uppercase());
            each
        } else {
            vec![character]
        };
        let found = candidates.iter().any(|candidate| {
            self.items.iter().any(|item| match item {
                ClassItem::Char(each) => each == candidate,
                ClassItem::Range(low, high) => (low..=high).contains(&candidate),
                ClassItem::Set(set, negated) => in_set(*set, *candidate) != *negated,
            })
        });
        found != self.negated
    }
}

/// Whether a character ends a line.
const fn is_line_terminator(character: char) -> bool {
    matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// The characters a match of a node must start with, when it must consume one first: past the
/// zero-width assertions, the first character, class-free.
fn first_characters(node: &Node) -> Option<Vec<char>> {
    match node {
        Node::Char(character) => Some(vec![*character]),
        Node::Group(inner, _) => first_characters(inner),
        Node::Alt(branches) => {
            let mut all = Vec::new();
            for branch in branches {
                all.extend(first_characters(branch)?);
            }
            Some(all)
        }
        Node::Repeat(inner, min, _, _) if *min > 0 => first_characters(inner),
        Node::Concat(nodes) => {
            for each in nodes {
                match each {
                    Node::Look(..) | Node::Boundary(_) | Node::Start | Node::End => {}
                    other => return first_characters(other),
                }
            }
            None
        }
        _ => None,
    }
}

/// The fewest and the most characters a node can match, the most unbounded when a repetition
/// is.
fn width(node: &Node) -> (usize, Option<usize>) {
    match node {
        Node::Char(_) | Node::Any | Node::Class(_) => (1, Some(1)),
        Node::Start | Node::End | Node::Boundary(_) | Node::Look(..) => (0, Some(0)),
        Node::Group(inner, _) => width(inner),
        Node::Concat(nodes) => {
            nodes
                .iter()
                .map(width)
                .fold((0, Some(0)), |(low, high), (each_low, each_high)| {
                    (low + each_low, high.zip(each_high).map(|(a, b)| a + b))
                })
        }
        Node::Alt(branches) => {
            let widths: Vec<(usize, Option<usize>)> = branches.iter().map(width).collect();
            let low = widths.iter().map(|each| each.0).min().unwrap_or(0);
            let high = widths
                .iter()
                .try_fold(0, |most, each| each.1.map(|high| most.max(high)));
            (low, high)
        }
        Node::Repeat(inner, min, max, _) => {
            let (low, high) = width(inner);
            (low * min, high.zip(*max).map(|(a, b)| a * b))
        }
    }
}

/// The groups' ranges while matching.
type Groups = Vec<Option<(usize, usize)>>;

impl Pattern {
    /// A pattern from its source and flags (`i`, `m`; the others change nothing here).
    #[must_use]
    pub fn new(source: &str, flags: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let mut parser = Parser {
            chars: &chars,
            at: 0,
            groups: 0,
            names: Vec::new(),
        };
        let root = parser.alternation();
        let first = first_characters(&root);
        Self {
            root,
            ignore_case: flags.contains('i'),
            multiline: flags.contains('m'),
            groups: parser.groups,
            names: parser.names,
            first,
        }
    }

    /// The index of a named group.
    #[must_use]
    pub fn group(&self, name: &str) -> Option<usize> {
        self.names
            .iter()
            .find(|(each, _)| each == name)
            .map(|(_, index)| *index)
    }

    /// The first match starting at or after an offset.
    #[must_use]
    pub fn find_at(&self, text: &[char], from: usize) -> Option<Match> {
        (from..=text.len()).find_map(|start| self.match_at(text, start))
    }

    /// The match starting exactly at an offset.
    #[must_use]
    pub fn match_at(&self, text: &[char], start: usize) -> Option<Match> {
        let possible = self.first.as_ref().is_none_or(|first| {
            text.get(start)
                .is_some_and(|each| first.iter().any(|wanted| self.same(*wanted, *each)))
        });
        if !possible {
            return None;
        }
        let mut groups: Groups = vec![None; self.groups];
        let mut end = None;
        let found = self.node(&self.root, text, start, &mut groups, &mut |position, _| {
            end = Some(position);
            true
        });
        found.then(|| Match {
            start,
            end: end.unwrap_or(start),
            groups,
        })
    }

    /// Whether the pattern matches anywhere in a text.
    #[must_use]
    pub fn test(&self, text: &str) -> bool {
        let chars: Vec<char> = text.chars().collect();
        self.find_at(&chars, 0).is_some()
    }

    /// Every match, left to right, an empty one stepping one on (`String#matchAll`).
    #[must_use]
    pub fn find_all(&self, text: &[char]) -> Vec<Match> {
        let mut found = Vec::new();
        let mut from = 0;
        while from <= text.len() {
            let Some(each) = self.find_at(text, from) else {
                break;
            };
            from = if each.end == each.start {
                each.end + 1
            } else {
                each.end
            };
            found.push(each);
        }
        found
    }

    /// Whether a character matches one of the pattern, with the `i` flag.
    fn same(&self, wanted: char, character: char) -> bool {
        wanted == character || (self.ignore_case && fold(wanted) == fold(character))
    }

    /// Whether a node matches at a position, then the rest does.
    fn node(
        &self,
        node: &Node,
        text: &[char],
        at: usize,
        groups: &mut Groups,
        rest: &mut dyn FnMut(usize, &mut Groups) -> bool,
    ) -> bool {
        match node {
            Node::Char(wanted) => {
                text.get(at).is_some_and(|each| self.same(*wanted, *each)) && rest(at + 1, groups)
            }
            Node::Any => {
                text.get(at).is_some_and(|each| !is_line_terminator(*each)) && rest(at + 1, groups)
            }
            Node::Class(class) => {
                text.get(at)
                    .is_some_and(|each| class.holds(*each, self.ignore_case))
                    && rest(at + 1, groups)
            }
            Node::Start => {
                (at == 0 || (self.multiline && is_line_terminator(text[at - 1])))
                    && rest(at, groups)
            }
            Node::End => {
                (at == text.len() || (self.multiline && is_line_terminator(text[at])))
                    && rest(at, groups)
            }
            Node::Boundary(wanted) => {
                let before = at > 0 && is_word(text[at - 1]);
                let after = text.get(at).is_some_and(|each| is_word(*each));
                ((before != after) == *wanted) && rest(at, groups)
            }
            Node::Group(inner, index) => {
                let saved = index.map(|index| groups[index]);
                let found = self.node(inner, text, at, groups, &mut |end, groups: &mut Groups| {
                    if let Some(index) = index {
                        let before = groups[*index];
                        groups[*index] = Some((at, end));
                        if rest(end, groups) {
                            return true;
                        }
                        groups[*index] = before;
                        false
                    } else {
                        rest(end, groups)
                    }
                });
                if !found && let (Some(index), Some(saved)) = (index, saved) {
                    groups[*index] = saved;
                }
                found
            }
            Node::Concat(nodes) => self.sequence(nodes, text, at, groups, rest),
            Node::Alt(branches) => branches
                .iter()
                .any(|branch| self.node(branch, text, at, groups, rest)),
            Node::Repeat(inner, min, max, greedy) => {
                self.repeat(inner, (*min, *max, *greedy, 0), text, at, groups, rest)
            }
            Node::Look(inner, ahead, negated) => {
                let found = if *ahead {
                    self.node(inner, text, at, groups, &mut |_, _| true)
                } else {
                    let (shortest, longest) = width(inner);
                    let lowest = longest.map_or(0, |longest| at.saturating_sub(longest));
                    at >= shortest
                        && (lowest..=at - shortest).rev().any(|start| {
                            self.node(inner, text, start, groups, &mut |end, _| end == at)
                        })
                };
                if found == *negated {
                    false
                } else {
                    rest(at, groups)
                }
            }
        }
    }

    /// Whether a sequence matches from its first node, then the rest does.
    fn sequence(
        &self,
        nodes: &[Node],
        text: &[char],
        at: usize,
        groups: &mut Groups,
        rest: &mut dyn FnMut(usize, &mut Groups) -> bool,
    ) -> bool {
        match nodes.split_first() {
            None => rest(at, groups),
            Some((first, others)) => self.node(first, text, at, groups, &mut |next, groups| {
                self.sequence(others, text, next, groups, rest)
            }),
        }
    }

    /// Whether a quantified node matches a further time or stops, greedy or lazy.
    fn repeat(
        &self,
        inner: &Node,
        (min, max, greedy, count): (usize, Option<usize>, bool, usize),
        text: &[char],
        at: usize,
        groups: &mut Groups,
        rest: &mut dyn FnMut(usize, &mut Groups) -> bool,
    ) -> bool {
        let can_stop = count >= min;
        let can_go = max.is_none_or(|max| count < max);
        let again = |groups: &mut Groups, rest: &mut dyn FnMut(usize, &mut Groups) -> bool| {
            can_go
                && self.node(inner, text, at, groups, &mut |next, groups| {
                    (next != at || count < min)
                        && self.repeat(
                            inner,
                            (min, max, greedy, count + 1),
                            text,
                            next,
                            groups,
                            rest,
                        )
                })
        };
        if greedy {
            if again(groups, rest) {
                return true;
            }
            can_stop && rest(at, groups)
        } else {
            if can_stop && rest(at, groups) {
                return true;
            }
            again(groups, rest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;

    /// The matched texts of a pattern over a text.
    fn matches(source: &str, flags: &str, text: &str) -> Vec<String> {
        let chars: Vec<char> = text.chars().collect();
        Pattern::new(source, flags)
            .find_all(&chars)
            .into_iter()
            .map(|each| chars[each.start..each.end].iter().collect())
            .collect()
    }

    #[test]
    fn matches_literals_classes_and_quantifiers() {
        assert_eq!(matches("a+b", "", "caab ab b"), ["aab", "ab"]);
        assert_eq!(matches("[\\d\\-]{2,3}", "", "1-2-3-4"), ["1-2", "-3-"]);
        assert_eq!(matches("x*?y", "", "xxy"), ["xxy"]);
        assert_eq!(
            matches("node\\.?js", "i", "NodeJS node.js"),
            ["NodeJS", "node.js"]
        );
        assert_eq!(matches("[^\\s,]+", "", "a b,c"), ["a", "b", "c"]);
    }

    #[test]
    fn reads_groups_alternation_and_lookaround() {
        assert_eq!(
            matches("(?:mac\\s?os(?!\\s?x)|(mac\\s?)?os\\s?x)", "i", "macOS osx"),
            ["macOS", "osx"]
        );
        assert_eq!(matches("(?<![\\w.])b", "", "ab .b b"), ["b"]);
        assert_eq!(matches("\\bci\\b", "", "ci cia"), ["ci"]);
        assert_eq!(matches("(?<=\\$)\\d+", "", "a$12 34"), ["12"]);
        assert_eq!(matches("^x", "m", "x\nx"), ["x", "x"]);
        assert_eq!(matches("\\p{Letter}+", "", "été 12"), ["été"]);
        let pattern = Pattern::new("(?<p>https?)://(?<a>[^/]+)", "");
        let text: Vec<char> = "go http://a.b/c".chars().collect();
        let found = pattern.find_at(&text, 0).expect("a match");
        assert_eq!(
            found.groups[pattern.group("a").expect("a group")],
            Some((10, 13))
        );
        assert!(Pattern::new("^\\s*$", "").test("  "));
    }
}
