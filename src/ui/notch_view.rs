use gpui::{
    Context, DispatchPhase, IntoElement, MouseExitEvent, MouseMoveEvent, Render, Window, canvas,
    div, prelude::*, px, rgb, size,
};

use super::pet::pet;
use super::session_card::session_card;
use crate::common::constants::constants;
use crate::enums::island_mode::IslandMode;
use crate::enums::session_phase::SessionPhase;
use crate::models::session::Session;
use crate::socket;

/// 灵动岛的根视图，保存悬停和展开状态。
pub struct NotchView {
    hovered: bool,
    mode: IslandMode,
    look_x: f32,
    look_y: f32,
    sessions: Vec<Session>,
}

impl Default for NotchView {
    fn default() -> Self {
        Self {
            hovered: false,
            mode: IslandMode::Compact,
            look_x: 0.0,
            look_y: 0.0,
            sessions: Vec::<Session>::new(),
        }
    }
}

impl NotchView {
    /// 创建视图，并接收后台线程发送的一次演示状态。
    pub fn new(cx: &mut Context<Self>) -> Self {
        let (sender, receiver) = async_channel::unbounded::<(String, SessionPhase)>();
        /// 创建视图，启动 socket 监听并异步接收会话状态。
        socket::start_listener(sender).expect("启动 socket 监听失败");
        // 只在绑定成功后注册清理，避免启动失败时删除其它进程的 socket。
        cx.on_app_quit(|_view, _cx| async {
            if let Err(err) = std::fs::remove_file(constants::SOCKET_PATH) {
                eprintln!("清理 socket 文件失败：{err}");
            }
        })
        .detach();
        cx.spawn(async move |view, cx| {
            while let Ok((session_id, phase)) = receiver.recv().await {
                if view
                    .update(cx, |view, cx| {
                        view.update_session_phase(&session_id, phase);
                        cx.notify();
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();
        Self::default()
    }
    /// 注册窗口退出事件，清除悬停状态。
    fn mouse_exit_listener(&self, cx: &mut Context<Self>) -> impl IntoElement {
        // 离开整个窗口时，清除元素回调可能遗留的悬停状态。
        let on_exit = cx.listener(|view, _event: &MouseExitEvent, _window, cx| {
            if view.hovered {
                view.hovered = false;
                view.look_x = 0.0;
                view.look_y = 0.0;
                cx.notify();
            }
        });
        canvas(
            |_, _, _| (),
            move |_, _, window, _| {
                window.on_mouse_event(move |event: &MouseExitEvent, phase, window, cx| {
                    // 只在一个事件阶段处理，避免捕获和冒泡各执行一次
                    if phase == DispatchPhase::Bubble {
                        on_exit(event, window, cx);
                    }
                });
            },
        )
        .absolute() // 让监听元素不占用布局空间。
        .size_full()
    }

    /// 更新指定会话的状态，其它会话保持原样。
    fn update_session_phase(&mut self, session_id: &str, phase: SessionPhase) {
        if let Some(session) = self
            .sessions
            .iter_mut()
            .find(|session| session.id == session_id)
        {
            session.phase = phase;
        } else {
            // 当前模拟协议没有项目名，先用会话编号作为展示名称。
            self.sessions.push(Session {
                id: session_id.to_owned(),
                project_name: session_id.to_owned(),
                phase,
            });
        }
    }
}

impl Render for NotchView {
    /// 根据交互状态构建岛的尺寸和内容。
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let height = self.mode.height();
        let awaiting_count = self
            .sessions
            .iter()
            .filter(|session| session.phase == SessionPhase::AwaitingApproval)
            .count();
        let label = match self.mode {
            IslandMode::Expanded => {
                format!("Codex sessions ({})", self.sessions.len())
            }
            IslandMode::Compact if awaiting_count > 0 => {
                format!("{} awaiting approval", awaiting_count)
            }
            IslandMode::Compact if self.hovered => String::from("Click to expand"),
            IslandMode::Compact => {
                format!("{} sessions", self.sessions.len())
            }
        };

        let exit_listener = self.mouse_exit_listener(cx);

        div()
            .relative()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(exit_listener)
            .child(
                div()
                    .id("notch-island")
                    .w(px(280.0))
                    .h(px(height))
                    .rounded(px(24.0))
                    .bg(rgb(0x000000))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xffffff))
                    .on_mouse_move(cx.listener(|view, event: &MouseMoveEvent, window, cx| {
                        let window_size = window.viewport_size();
                        let point = event.position;
                        let width = f32::from(window_size.width);
                        let height = f32::from(window_size.height);
                        let mouse_x = f32::from(point.x);
                        let mouse_y = f32::from(point.y);
                        // 将窗口内横坐标映射为 -1 到 1，再换算成眼睛偏移。
                        let direction_x = (mouse_x / width * 2.0 - 1.0).clamp(-1.0, 1.0); // 限制结果，低于 -1 就取 -1，高于 1 就取 1。
                        let direction_y = (mouse_y / height * 2.0 - 1.0).clamp(-1.0, 1.0);
                        view.hovered = true;
                        view.look_x = direction_x * 3.0;
                        view.look_y = direction_y * 2.0;
                        cx.notify();
                    }))
                    .on_click(cx.listener(|view, _event, window, cx| {
                        if matches!(view.mode, IslandMode::Compact) {
                            view.mode = IslandMode::Expanded;
                            window.resize(size(px(280.0), px(view.mode.height())));
                            cx.notify();
                        }
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(pet(self.hovered, self.look_x, self.look_y))
                                    .child(label),
                            )
                            // 条件成立时，给当前元素追加布局或内容。
                            .when(matches!(self.mode, IslandMode::Expanded), |panel| {
                                panel
                                    .child(
                                        div().text_sm().text_color(rgb(0xa1a1aa)).child(
                                            div().flex().flex_col().gap_2().children(
                                                self.sessions
                                                    .iter()
                                                    .map(|session| session_card(session)),
                                            ),
                                        ),
                                    )
                                    .child(
                                        div()
                                            .id("collapse-button")
                                            .px_3() // 左右内边距
                                            .py_1() // 上下内边距
                                            .rounded_md() // 圆角
                                            .bg(rgb(0x27272a))
                                            .text_sm()
                                            .cursor_pointer()
                                            .on_click(cx.listener(|view, _event, window, cx| {
                                                // 收起后，不再让这次点击触发父元素的展开逻辑。
                                                cx.stop_propagation();
                                                view.mode = IslandMode::Compact;
                                                window.resize(size(
                                                    px(280.0),
                                                    px(view.mode.height()),
                                                ));
                                                cx.notify();
                                            }))
                                            .child("‌Collapse"),
                                    )
                            }),
                    ),
            )
    }
}
