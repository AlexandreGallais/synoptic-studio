//! What Unicorn's rules on event handlers share: the names and types of an event parameter, the
//! function a parameter belongs to, and whether a place of that function runs after it
//! suspended (`eslint-plugin-unicorn` 78.0.0, `shared/late-event-handler.js`,
//! `utils/is-event.js`).

use oxc_ast::AstKind;
use oxc_ast::ast::IdentifierReference;
use oxc_semantic::{NodeId, SymbolId};
use oxc_span::{GetSpan, Span};

use crate::rules::unicorn::annotations::{Class, Targets, class_of_symbol};
use crate::rules::unicorn::syntax::{descendants, symbol_of};
use crate::source::Source;

/// The React event types, also read from `react`'s exports.
const REACT_EVENTS: [&str; 20] = [
    "AnimationEvent",
    "BaseSyntheticEvent",
    "ChangeEvent",
    "ClipboardEvent",
    "CompositionEvent",
    "DragEvent",
    "FocusEvent",
    "FormEvent",
    "InputEvent",
    "InvalidEvent",
    "KeyboardEvent",
    "MouseEvent",
    "PointerEvent",
    "SubmitEvent",
    "SyntheticEvent",
    "TouchEvent",
    "ToggleEvent",
    "TransitionEvent",
    "UIEvent",
    "WheelEvent",
];

/// The DOM's event types, and React's.
const EVENTS: [&str; 82] = [
    "AnimationEvent",
    "AnimationPlaybackEvent",
    "AudioProcessingEvent",
    "BeforeUnloadEvent",
    "BlobEvent",
    "ClipboardEvent",
    "CloseEvent",
    "CommandEvent",
    "CompositionEvent",
    "ContentVisibilityAutoStateChangeEvent",
    "CookieChangeEvent",
    "CustomEvent",
    "DeviceMotionEvent",
    "DeviceOrientationEvent",
    "DragEvent",
    "ErrorEvent",
    "Event",
    "ExtendableCookieChangeEvent",
    "ExtendableEvent",
    "ExtendableMessageEvent",
    "FetchEvent",
    "FocusEvent",
    "FontFaceSetLoadEvent",
    "FormDataEvent",
    "GPUUncapturedErrorEvent",
    "GamepadEvent",
    "HashChangeEvent",
    "IDBVersionChangeEvent",
    "InputEvent",
    "KeyboardEvent",
    "MIDIConnectionEvent",
    "MIDIMessageEvent",
    "MediaEncryptedEvent",
    "MediaKeyMessageEvent",
    "MediaQueryListEvent",
    "MediaStreamTrackEvent",
    "MessageEvent",
    "MouseEvent",
    "NavigateEvent",
    "NavigationCurrentEntryChangeEvent",
    "NotificationEvent",
    "OfflineAudioCompletionEvent",
    "PageRevealEvent",
    "PageSwapEvent",
    "PageTransitionEvent",
    "PaymentMethodChangeEvent",
    "PaymentRequestUpdateEvent",
    "PictureInPictureEvent",
    "PointerEvent",
    "PopStateEvent",
    "ProgressEvent",
    "PromiseRejectionEvent",
    "PushEvent",
    "PushSubscriptionChangeEvent",
    "RTCDataChannelEvent",
    "RTCDTMFToneChangeEvent",
    "RTCErrorEvent",
    "RTCPeerConnectionIceErrorEvent",
    "RTCPeerConnectionIceEvent",
    "RTCTrackEvent",
    "RTCTransformEvent",
    "SecurityPolicyViolationEvent",
    "SpeechRecognitionErrorEvent",
    "SpeechRecognitionEvent",
    "SpeechSynthesisErrorEvent",
    "SpeechSynthesisEvent",
    "StorageEvent",
    "SubmitEvent",
    "TaskPriorityChangeEvent",
    "TextEvent",
    "ToggleEvent",
    "TouchEvent",
    "TrackEvent",
    "TransitionEvent",
    "UIEvent",
    "WebGLContextEvent",
    "WheelEvent",
    "BaseSyntheticEvent",
    "ChangeEvent",
    "FormEvent",
    "InvalidEvent",
    "SyntheticEvent",
];

/// How the event rules read an annotation (`utils/is-event.js`).
const EVENT_TARGETS: Targets = Targets {
    names: &EVENTS,
    non_targets: &[],
    constructors: &[],
    imports: &[("react", &REACT_EVENTS)],
    allow_nullish_in_mixed_union: false,
    mixed_union: Class::Unknown,
    prefer_definitions: true,
};

/// Whether a name is an event's: `e`, `event`, `evt`, or a camelCase name ending in `Event`.
#[must_use]
pub fn is_event_name(name: &str) -> bool {
    if matches!(name, "e" | "event" | "evt") {
        return true;
    }
    let Some(stem) = name.strip_suffix("Event") else {
        return false;
    };
    let mut characters = stem.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && characters.all(|each| each.is_ascii_alphanumeric())
}

/// What a symbol's annotation says of it as an event.
#[must_use]
pub fn event_class(source: &Source, symbol: SymbolId) -> Class {
    class_of_symbol(source, symbol, &EVENT_TARGETS, 0)
}

/// The function a name is a parameter of, and the parameter's symbol.
#[must_use]
pub fn handler_of(source: &Source, name: &IdentifierReference) -> Option<(NodeId, SymbolId)> {
    let symbol = symbol_of(source, name)?;
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let mut in_parameters = false;
    for ancestor in std::iter::once(nodes.get_node(declaration)).chain(nodes.ancestors(declaration))
    {
        match ancestor.kind() {
            AstKind::FormalParameters(_) => in_parameters = true,
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => {
                return in_parameters.then_some((ancestor.id(), symbol));
            }
            AstKind::CatchClause(_) | AstKind::VariableDeclarator(_) => return None,
            _ => {}
        }
    }
    None
}

/// The closest function holding a node, the node left out.
#[must_use]
pub fn enclosing_function(source: &Source, id: NodeId) -> Option<NodeId> {
    source
        .semantic
        .nodes()
        .ancestors(id)
        .find(|ancestor| {
            matches!(
                ancestor.kind(),
                AstKind::Function(_) | AstKind::ArrowFunctionExpression(_)
            )
        })
        .map(oxc_semantic::AstNode::id)
}

/// Whether a node suspends its function: `await`, `yield` or `for await`.
fn is_suspension(kind: &AstKind) -> bool {
    match kind {
        AstKind::AwaitExpression(_) | AstKind::YieldExpression(_) => true,
        AstKind::ForOfStatement(statement) => statement.r#await,
        _ => false,
    }
}

/// The ranges of the suspensions of a function, those of its nested functions left out.
#[must_use]
pub fn suspensions(source: &Source, function: NodeId) -> Vec<Span> {
    descendants(source, function)
        .filter(|node| is_suspension(&node.kind()))
        .filter(|node| enclosing_function(source, node.id()) == Some(function))
        .map(|node| node.kind().span())
        .collect()
}

/// Whether a node of a function runs after it suspended: a suspension of it ends before the
/// node is read (an offset), or the node repeats in a loop of the function holding one.
#[must_use]
pub fn runs_late(source: &Source, id: NodeId, function: NodeId, read_at: u32) -> bool {
    let suspended = suspensions(source, function);
    if suspended.iter().any(|span| span.end <= read_at) {
        return true;
    }
    let nodes = source.semantic.nodes();
    let mut child = id;
    for ancestor in nodes.ancestors(id) {
        if ancestor.id() == function {
            break;
        }
        let child_span = nodes.kind(child).span();
        let repeated = match ancestor.kind() {
            AstKind::ForStatement(statement) => {
                statement
                    .test
                    .as_ref()
                    .is_some_and(|test| test.span() == child_span)
                    || statement
                        .update
                        .as_ref()
                        .is_some_and(|update| update.span() == child_span)
                    || statement.body.span() == child_span
            }
            AstKind::ForInStatement(statement) => statement.right.span() != child_span,
            AstKind::ForOfStatement(statement) => statement.right.span() != child_span,
            AstKind::WhileStatement(_) | AstKind::DoWhileStatement(_) => true,
            _ => false,
        };
        let span = ancestor.kind().span();
        if repeated
            && suspended
                .iter()
                .any(|each| each.start >= span.start && each.end <= span.end)
        {
            return true;
        }
        child = ancestor.id();
    }
    false
}

/// The `addEventListener(type, listener, options)` call a function is the listener of, through
/// TypeScript's wrappers, with its options' object.
#[must_use]
pub fn registration<'a>(
    source: &Source<'_, 'a>,
    function: NodeId,
) -> Option<(
    &'a oxc_ast::ast::CallExpression<'a>,
    &'a oxc_ast::ast::ObjectExpression<'a>,
)> {
    let nodes = source.semantic.nodes();
    let mut child = function;
    let mut parent = nodes.parent_node(child);
    while matches!(
        parent.kind(),
        AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_)
    ) {
        child = parent.id();
        parent = nodes.parent_node(child);
    }
    let AstKind::CallExpression(call) = parent.kind() else {
        return None;
    };
    let listener_span = nodes.kind(child).span();
    let callee = crate::rules::unicorn::syntax::unwrap(&call.callee);
    let is_add = match callee {
        oxc_ast::ast::Expression::Identifier(name) => name.name == "addEventListener",
        _ => super::nodes::static_property_name(callee).as_deref() == Some("addEventListener"),
    };
    if call.arguments.len() != 3 || call.arguments[1].span() != listener_span || !is_add {
        return None;
    }
    match crate::rules::unicorn::syntax::unwrap(call.arguments[2].as_expression()?) {
        oxc_ast::ast::Expression::ObjectExpression(options) => Some((call, options)),
        _ => None,
    }
}

/// The value of the last plain property of an options' object with a name, when every property
/// is plain: neither spread nor computed.
#[must_use]
pub fn option<'b, 'a>(
    options: &'b oxc_ast::ast::ObjectExpression<'a>,
    name: &str,
) -> Option<Option<&'b oxc_ast::ast::Expression<'a>>> {
    let mut found = None;
    for property in &options.properties {
        let oxc_ast::ast::ObjectPropertyKind::ObjectProperty(property) = property else {
            return None;
        };
        if property.computed {
            return None;
        }
        if property.key.is_specific_static_name(name) {
            found = Some(&property.value);
        }
    }
    Some(found)
}
