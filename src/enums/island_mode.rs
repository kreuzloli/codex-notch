/// 岛的显示状态，与 Codex 任务状态分开。
#[derive(Default, Clone, Copy)]
pub(crate) enum IslandMode {
    #[default]
    Compact,
    Expanded,
}

impl IslandMode {
    /// 返回当前状态对应的高度
    pub(crate) fn height(self) -> f32 {
        match self {
            Self::Compact => 80.0,
            Self::Expanded => 240.0,
        }
    }
}
