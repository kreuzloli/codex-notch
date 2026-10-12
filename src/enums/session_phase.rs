/// Codex 会话的业务状态，与岛的展开、收起状态分开。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SessionPhase {
    Idle,
    Thinking,
    Working,
    AwaitingApproval,
    Finished,
    Error,
}

impl SessionPhase {
    /// 返回界面展示的状态文字。
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Thinking => "Thinking",
            Self::Working => "Working",
            Self::AwaitingApproval => "Awaiting Approval",
            Self::Finished => "Finished",
            Self::Error => "Error",
        }
    }
}
