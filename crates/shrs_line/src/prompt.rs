//! Shell prompt

use crossterm::style::{ContentStyle, StyledContent};
use shrs_utils::styled_buf::StyledBuf;

use crate::line::LineCtx;

/// Implement this trait to create your own prompt
pub trait Prompt {
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf;
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf;
}

/// Default implementation for [Prompt]
pub struct DefaultPrompt {}

impl DefaultPrompt {
    pub fn new() -> Self {
        DefaultPrompt {}
    }
}

impl Prompt for DefaultPrompt {
    // TODO i still don't like passing all this context down
    fn prompt_left(&self, _line_ctx: &mut LineCtx) -> StyledBuf {
        StyledBuf::from_iter(vec![StyledContent::new(
            ContentStyle::new(),
            "> ".to_string(),
        )])
    }

    fn prompt_right(&self, _line_ctx: &mut LineCtx) -> StyledBuf {
        StyledBuf::from_iter(vec![])
    }
}
