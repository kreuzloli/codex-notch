use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px, rgb};

/// 灵动岛的根视图；暂时没有需要保存的状态。
#[derive(Default)]
pub(crate) struct NotchView {
    hovered: bool,
}

impl Render for NotchView {
    /// 外层保留透明背景，内层绘制可见的岛。
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let label = if self.hovered {
            "Click to expand"
        } else {
            "Codex Notch"
        };
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(280.0))
                    .h(px(80.0))
                    .rounded(px(24.0))
                    .bg(rgb(0x000000))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xffffff))
                    .child(
                        div()
                            .id("notch-island")
                            .w(px(280.0))
                            .h(px(80.0))
                            .rounded(px(24.0))
                            .bg(rgb(0x000000))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xffffff))
                            .on_hover(cx.listener(|view, hovered, _window, cx| {
                                view.hovered = *hovered;
                                cx.notify();
                            }))
                            .child(label),
                    ),
            )
    }
}
