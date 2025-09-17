use axum::{extract::State, response::Html};
use std::sync::OnceLock;

use crate::AppState;

static TEMPLATE_OUTCOME: OnceLock<String> = OnceLock::new();

pub async fn serve_index(State(state): State<AppState>) -> Html<&'static str> {
    Html(TEMPLATE_OUTCOME.get_or_init(|| {
        include_str!("../../static/index.html")
            .replace("[TEMPLATE:PAGE_TITLE]", &state.page_title)
            .replace("[TEMPLATE:PAGE_OWNER_NAME]", &state.page_owner_name)
            .replace(
                "[TEMPLATE:QUESTION_RESPOND_TEXT]",
                &state.page_question_respond_text,
            )
            .replace(
                "[TEMPLATE:QUESTION_RESPOND_URL]",
                state.page_question_respond_url.as_str(),
            )
            .replace(
                "[TEMPLATE:PLACEHOLDER_QUESTION]",
                &state.page_question_placeholder,
            )
            .replace(
                "[TEMPLATE:QUESTION_MIN_LENGTH]",
                &state.page_question_min_length.to_string(),
            )
            .replace(
                "[TEMPLATE:QUESTION_MAX_LENGTH]",
                &state.page_question_max_length.to_string(),
            )
    }))
}
