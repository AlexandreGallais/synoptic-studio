//! `@angular-eslint/no-output-native`: no output named, nor aliased, as a standard DOM event,
//! which a template could not tell from the element's (`@angular-eslint/eslint-plugin` 22.6.0,
//! the cases of its tests); the events are the plugin's list, `getNativeEventNames`, drawn
//! from MDN's browser compatibility data.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::bindings::{entry_parts, output_names};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoOutputNative;

/// The standard DOM events (`getNativeEventNames`).
const NATIVE_EVENTS: [&str; 181] = [
    "abort",
    "afterprint",
    "afterscriptexecute",
    "animationcancel",
    "animationend",
    "animationiteration",
    "animationstart",
    "appinstalled",
    "auxclick",
    "beforeinput",
    "beforeinstallprompt",
    "beforematch",
    "beforeprint",
    "beforescriptexecute",
    "beforetoggle",
    "beforeunload",
    "beforexrselect",
    "beginEvent",
    "blur",
    "cancel",
    "canplay",
    "canplaythrough",
    "change",
    "click",
    "close",
    "command",
    "compositionend",
    "compositionstart",
    "compositionupdate",
    "contentvisibilityautostatechange",
    "contextlost",
    "contextmenu",
    "contextrestored",
    "copy",
    "cuechange",
    "cut",
    "dblclick",
    "devicemotion",
    "deviceorientation",
    "deviceorientationabsolute",
    "DOMActivate",
    "DOMContentLoaded",
    "DOMMouseScroll",
    "drag",
    "dragend",
    "dragenter",
    "dragexit",
    "dragleave",
    "dragover",
    "dragstart",
    "drop",
    "durationchange",
    "emptied",
    "encrypted",
    "ended",
    "endEvent",
    "enterpictureinpicture",
    "error",
    "focus",
    "focusin",
    "focusout",
    "formdata",
    "freeze",
    "fullscreenchange",
    "fullscreenerror",
    "gamepadconnected",
    "gamepaddisconnected",
    "gesturechange",
    "gestureend",
    "gesturestart",
    "gotpointercapture",
    "hashchange",
    "input",
    "installresult",
    "interest",
    "invalid",
    "keydown",
    "keypress",
    "keyup",
    "languagechange",
    "leavepictureinpicture",
    "load",
    "loadeddata",
    "loadedmetadata",
    "loadstart",
    "location",
    "loseinterest",
    "lostpointercapture",
    "message",
    "messageerror",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "mousewheel",
    "MozMousePixelScroll",
    "offline",
    "online",
    "orientationchange",
    "pagehide",
    "pagereveal",
    "pageshow",
    "pageswap",
    "paste",
    "pause",
    "play",
    "playing",
    "pointercancel",
    "pointerdown",
    "pointerenter",
    "pointerleave",
    "pointerlockchange",
    "pointerlockerror",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerrawupdate",
    "pointerup",
    "popstate",
    "prerenderingchange",
    "progress",
    "promptaction",
    "promptdismiss",
    "ratechange",
    "readystatechange",
    "rejectionhandled",
    "repeatEvent",
    "reset",
    "resize",
    "resume",
    "scroll",
    "scrollend",
    "scrollsnapchange",
    "scrollsnapchanging",
    "search",
    "securitypolicyviolation",
    "seeked",
    "seeking",
    "select",
    "selectionchange",
    "selectstart",
    "slotchange",
    "stalled",
    "storage",
    "stream",
    "submit",
    "suspend",
    "timeupdate",
    "toggle",
    "touchcancel",
    "touchend",
    "touchmove",
    "touchstart",
    "track",
    "transitioncancel",
    "transitionend",
    "transitionrun",
    "transitionstart",
    "unhandledrejection",
    "unload",
    "validationstatuschange",
    "visibilitychange",
    "volumechange",
    "vrdisplayactivate",
    "vrdisplayconnect",
    "vrdisplaydeactivate",
    "vrdisplaydisconnect",
    "vrdisplaypresentchange",
    "waiting",
    "waitingforkey",
    "webglcontextcreationerror",
    "webglcontextlost",
    "webglcontextrestored",
    "webkitmouseforcechanged",
    "webkitmouseforcedown",
    "webkitmouseforceup",
    "webkitmouseforcewillbegin",
    "wheel",
];

impl Rule for NoOutputNative {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-output-native"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        for name in output_names(class) {
            let (property, alias) = entry_parts(name.text, false);
            if NATIVE_EVENTS.contains(&property.as_str())
                || alias.is_some_and(|alias| NATIVE_EVENTS.contains(&alias.as_str()))
            {
                findings.push(source.finding(
                    self.name(),
                    name.at,
                    "Output bindings, including aliases, should not be named as standard DOM events".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NATIVE_EVENTS, NoOutputNative};
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoOutputNative, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn holds_the_plugin_s_list_sorted_as_there() {
        // The plugin sorts its list by `localeCompare`: case aside, in order.
        let lower: Vec<String> = NATIVE_EVENTS
            .iter()
            .map(|each| each.to_lowercase())
            .collect();
        assert!(lower.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn f51_ac2_accepts_outputs_named_otherwise() {
        for code in [
            "@Component({ outputs: ['userSelected', 'a: itemDeleted'] })\nclass Test {}\n",
            "@Component()\nclass Test {\n  readonly changed = output();\n  @Output() selected = new EventEmitter();\n  click = 1;\n}\n",
            // A property named as an event without being an output.
            "@Component()\nclass Test {\n  @Input() change: string;\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_an_output_named_as_an_event() {
        // The name at column 13 after `  @Output() `, 12 after `  readonly `; the alias's quote.
        assert_eq!(
            check("@Component()\nclass Test {\n  @Output() change = new EventEmitter();\n}\n"),
            [(3, 13)]
        );
        assert_eq!(
            check("@Component()\nclass Test {\n  readonly click = output();\n}\n"),
            [(3, 12)]
        );
        assert_eq!(
            check(
                "@Component()\nclass Test {\n  @Output('blur') blurred = new EventEmitter();\n}\n"
            ),
            [(3, 11)]
        );
        assert_eq!(
            check(
                "@Component()\nclass Test {\n  readonly done = output({ alias: 'select' });\n}\n"
            ),
            [(3, 35)]
        );
        // `@Component({ outputs: [` is 23 columns: `'a: copy'` at 24, `'paste'` 11 further.
        assert_eq!(
            check("@Component({ outputs: ['a: copy', 'paste'] })\nclass Test {}\n"),
            [(1, 24), (1, 35)]
        );
    }
}
