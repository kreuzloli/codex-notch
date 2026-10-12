use gpui::{FontWeight, IntoElement, ParentElement, Styled, div, px, rgb};

use crate::enums::session_phase::SessionPhase;
use crate::models::session::Session;

/// 根据会话数据构建一条展示行。
pub(crate) fn session_card(session: &Session) -> impl IntoElement {
    let status_color = match session.phase {
        SessionPhase::Idle => rgb(0xa1a1aa),
        SessionPhase::Thinking => rgb(0xa78bfa),
        SessionPhase::Working => rgb(0x60a5fa),
        SessionPhase::AwaitingApproval => rgb(0xfbbf24),
        SessionPhase::Finished => rgb(0x4ade80),
        SessionPhase::Error => rgb(0xf87171),
    };

    div()
        .w(px(256.0))
        .px_3()
        .py_2()
        .rounded_md()
        .bg(rgb(0x18181b))
        .flex()
        .items_center()
        .justify_center()
        .gap_3()
        .font_weight(FontWeight::SEMIBOLD)
        .text_sm()
        .text_color(rgb(0xffffff))
        .child(session.project_name.clone())
        .child(div().text_color(status_color).child(session.phase.label()))
}
