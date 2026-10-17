//! Tests of the run's subagents read from their files (EN-087).

use progress_page::agents::{
    IDLE_MS, agent_of, first_time, has_ended, is_running, story_in, tokens_of_file,
};
use run_totals::tally::{FileTally, Tokens};
use run_totals::totals::Cache;

const META: &str = r#"{"agentType":"story-worker","description":"EN-087 progress artifact","toolUseId":"toolu_1"}"#;
const HEAD: &str = "{\"type\":\"queue\"}\n{\"type\":\"user\",\"timestamp\":\"2026-10-17T19:50:00.000Z\"}\n{\"timestamp\":\"2026-10-17T19:51:00.000Z\"}\n";
// The call, the attachment of the hook that checked it (EN-088), then its result.
const HANDED_BACK: &str = "{\"type\":\"assistant\",\"message\":{\"content\":[{\"type\":\"tool_use\",\"name\":\"SubagentHandback\"}]}}\n\
{\"attachment\":{\"type\":\"hook_success\",\"hookName\":\"PreToolUse:SubagentHandback\"}}\n\
{\"type\":\"user\",\"toolUseResult\":{\"success\":true}}\n";
const WORKING: &str = "{\"type\":\"assistant\",\"message\":{\"stop_reason\":\"tool_use\",\"content\":[{\"type\":\"tool_use\",\"name\":\"Bash\"}]}}\n";

#[test]
fn finds_the_story_a_description_names() {
    assert_eq!(
        story_in("EN-087 progress artifact").as_deref(),
        Some("EN-087")
    );
    assert_eq!(
        story_in("story-worker: finish AUD-022's fixes").as_deref(),
        Some("AUD-022")
    );
    // A type with a dash, no id at all.
    assert_eq!(story_in("story-worker"), None);
    assert_eq!(story_in(""), None);
}

#[test]
fn reads_the_first_time_of_a_transcript() {
    // 2026-10-17 19:50:00 UTC: 20,743 days after 1970-01-01, then 19 h 50 min.
    // 20,743 × 86,400,000 + 71,400,000 = 1,792,195,200,000 + 71,400,000 = 1,792,266,600,000.
    assert_eq!(first_time(HEAD), 1_792_266_600_000);
    assert_eq!(first_time("{\"type\":\"queue\"}\nnot json\n"), 0);
}

#[test]
fn tells_an_agent_done_from_one_still_working() {
    assert!(has_ended(HANDED_BACK));
    assert!(has_ended(
        "{\"type\":\"assistant\",\"message\":{\"stop_reason\":\"end_turn\"}}\n{\"type\":\"system\"}\n"
    ));
    assert!(!has_ended(WORKING));
    assert!(!has_ended(""));
    // A line cut by the read before the handback: not read, so the agent still works.
    assert!(!has_ended(
        "ype\":\"assistant\",\"message\":{\"stop_reason\":\"end_turn\"}}\n{\"type\":\"user\"}\n"
    ));
    // A handback followed by another assistant line: the agent was resumed.
    assert!(!has_ended(&format!("{HANDED_BACK}{WORKING}")));
    assert!(has_ended(&format!("{WORKING}{HANDED_BACK}")));
}

#[test]
fn f20_ac5_reads_an_agent_and_whether_it_runs() {
    let last = 1_792_266_700_000;
    let agent = agent_of("a1", META, HEAD, WORKING, last);

    assert_eq!(agent.kind, "story-worker");
    assert_eq!(agent.story.as_deref(), Some("EN-087"));
    assert_eq!(agent.start, 1_792_266_600_000);
    assert!(is_running(&agent, last + IDLE_MS));
    // Idle for longer than IDLE_MS: taken as stopped.
    assert!(!is_running(&agent, last + IDLE_MS + 1));
    assert!(!is_running(
        &agent_of("a2", META, HEAD, HANDED_BACK, last),
        last
    ));

    let unknown = agent_of("a3", "not json", "", "", last);

    assert_eq!((unknown.kind.as_str(), unknown.story), ("", None));
}

#[test]
fn takes_an_agents_tokens_from_the_run_cache_by_file_name() {
    let tokens = Tokens {
        input: 1,
        cache_write: 20,
        cache_read: 300,
        output: 4,
    };
    let cache = Cache {
        start: 1,
        end: 0,
        files: vec![(
            "/home/x/.claude/projects/p/s1/subagents/agent-a1.jsonl".to_owned(),
            FileTally {
                tokens,
                ..FileTally::default()
            },
        )],
    };

    assert_eq!(tokens_of_file(&cache, "agent-a1.jsonl"), tokens);
    assert_eq!(tokens_of_file(&cache, "agent-a2.jsonl"), Tokens::default());
}
