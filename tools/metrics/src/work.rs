//! Units of work of the metrics report (`docs/tooling/metrics.md`): what the agent's requests and
//! tool calls were spent on, with active time and tokens.

/// A pause longer than this is not work: a gap counts for this much at most, 5 minutes.
pub const IDLE_MS: i64 = 300_000;

/// Active time of a series of moments: the gaps between them, each capped at [`IDLE_MS`].
#[must_use]
pub fn active_ms(times: &[i64]) -> i64 {
    let mut sorted = times.to_vec();

    sorted.sort_unstable();
    sorted
        .windows(2)
        .map(|pair| (pair[1] - pair[0]).min(IDLE_MS))
        .sum()
}
