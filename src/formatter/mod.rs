use crate::ai::repository::CommitMessage;

pub fn format_commit_message(commit: &CommitMessage, format_template: &str) -> String {
    let base = format_template
        .replace("{type}", &commit.commit_type)
        .replace("{message}", &commit.message);

    match &commit.scope {
        Some(scope) if !scope.is_empty() => {
            base.replace("{scope}", scope)
        }
        _ => {
            // If scope is None or empty, remove the placeholder and potential surrounding parentheses.
            base.replace("({scope})", "").replace("{scope}", "").trim().replace("  ", " ")
        }
    }
}
