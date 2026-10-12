use std::time::Duration;

use gpui::{
    Animation, AnimationExt, IntoElement, ParentElement, Styled, bounce, div, ease_in_out, px, rgb,
};

/// 构建静态宠物：圆角身体和两只眼睛。
pub(crate) fn pet(hovered: bool, look_x: f32, look_y: f32) -> impl IntoElement {
    div()
        .w(px(40.0))
        .h(px(40.0))
        .rounded(px(14.0))
        .bg(rgb(0xededef))
        .flex()
        .items_center()
        .justify_center()
        .child(eyes(hovered, look_x, look_y))
        .with_animation(
            "pet-breathing",
            Animation::new(Duration::from_secs(2))
                .repeat() // 循环播放
                // bounce 让进度从 0 → 1 → 0 往返，这里不是物理弹跳
                .with_easing(bounce(ease_in_out)),
            |body, progress| {
                // progress 经过缓动处理后的进度，用来计算当前高度
                body.h(px(40.0 + 2.0 * progress))
            },
        )
}

/// 两只眼睛共用一个动画进度，保持眨眼同步。
fn eyes(hovered: bool, look_x: f32, look_y: f32) -> impl IntoElement {
    div()
        .relative()
        .left(px(look_x))
        .top(px(look_y))
        .flex()
        .items_center()
        .gap(px(8.0))
        .with_animation(
            "pet-blinking",
            Animation::new(Duration::from_secs(4)).repeat(),
            move |row, progress| {
                // 周期前 94% 保持睁眼，最后 6% 完成闭眼和睁眼。
                let openness = if progress < 0.94 {
                    1.0
                } else {
                    let blink_progress = (progress - 0.94) / 0.06;
                    1.0 - bounce(ease_in_out)(blink_progress)
                };

                row.child(eye(hovered, openness))
                    .child(eye(hovered, openness))
            },
        )
}

/// openness 为 1 时睁眼，为 0 时收成一条细线。
fn eye(hovered: bool, openness: f32) -> impl IntoElement {
    let width = if hovered { 8.0 } else { 5.0 };
    let open_height = if hovered { 13.0 } else { 10.0 };
    let height = 1.0 + (open_height - 1.0) * openness;
    div()
        .w(px(width))
        .h(px(height))
        .rounded(px(width / 2.0))
        .bg(rgb(0x1A1412))
}
