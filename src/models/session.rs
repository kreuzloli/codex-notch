use crate::enums::session_phase::SessionPhase;

/// 展开面板中一条会话的数据。
#[derive(Debug)]
pub(crate) struct Session {
    pub(crate) id: String,
    pub(crate) project_name: String,
    pub(crate) phase: SessionPhase,
}
