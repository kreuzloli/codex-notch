mod ui;
use gpui::{
    App, AppContext, Application, Bounds, WindowBackgroundAppearance, WindowBounds, WindowKind,
    WindowOptions, point, px, size,
};
use ui::NotchView;

/// 启动事件循环，并创建承载根视图的窗口。
/// - Application::new().run(...)：启动应用事件循环，持续处理点击、窗口和绘制事件。
/// - cx.new(|_| NotchView)：创建由 GPUI 管理的视图实体。闭包返回 NotchView，其所有权交给 GPUI 管理。
fn main() {
    println!("Codex Notch Start!");
    Application::new().run(|cx: &mut App| {
        let display = cx.primary_display().expect("Primary display not found");
        let screen = display.bounds();
        let window_size = size(px(280.0), px(80.0));
        // 屏幕原点不一定是 (0, 0)，计算位置时保留它。
        let origin = point(
            screen.origin.x + (screen.size.width - window_size.width) / 2.0,
            screen.origin.y + px(40.0),
        );
        // let bounds = Bounds::centered(None, size(px(480.0), px(240.0)), cx);
        let bounds = Bounds::new(origin, window_size);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)), // 窗口的位置和大小
                titlebar: None,                                      // 不显示常规标题栏
                kind: WindowKind::PopUp,                             // 创建弹出式窗口
                focus: false,                                        // 创建时不获取键盘焦点
                is_movable: false,                                   // 禁止系统提供的窗口移动行为
                is_resizable: false,                                 // 禁止用户手动调整窗口大小
                is_minimizable: false,                               // 禁止最小化窗口
                window_background: WindowBackgroundAppearance::Transparent, // 背景透明
                ..Default::default()
            },
            |_, cx| cx.new(|_| NotchView::default()),
        )
        .expect("Init window failed");
        // cx.activate(true); 把应用切到前台。
    });
}
