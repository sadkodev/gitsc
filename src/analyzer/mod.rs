use crate::config::SmartCommitConfig;
use log::debug;

pub fn analyze_diff(diff: &str, smart_commit_config: &SmartCommitConfig) -> String {
    let lines: Vec<&str> = diff.lines().collect();
    let threshold = smart_commit_config.line_threshold as usize;

    if lines.len() > threshold {
        debug!(
            "Diff exceeds line threshold ({} > {}). Truncating diff.",
            lines.len(),
            threshold
        );
        let truncated_diff = lines[..threshold].join("\n");
        format!(
            "{}
... [Diff truncated to {} lines] ...",
            truncated_diff, threshold
        )
    } else {
        diff.to_string()
    }
}
