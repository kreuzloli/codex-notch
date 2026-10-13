use std::{
    io::{self, BufRead, BufReader, Write},
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use crate::{common::constants::constants, enums::session_phase::SessionPhase};
use async_channel::Sender;

/// 后台向界面发送的一次会话更新。
pub(crate) struct SessionUpdate {
    pub(crate) session_id: String,
    pub(crate) project_name: String,
    pub(crate) phase: SessionPhase,
}

/// 解析模拟协议：会话编号|项目名|状态。
pub(crate) fn parse_session_update(text: &str) -> Result<SessionUpdate, &'static str> {
    let Some((session_id, remaining)) = text.split_once('|') else {
        return Err("invalid format");
    };
    let Some((project_name, phase_text)) = remaining.split_once('|') else {
        return Err("invalid format");
    };
    if session_id.is_empty() {
        return Err("empty session id");
    }
    if project_name.is_empty() {
        return Err("empty project name");
    }
    let Some(phase) = SessionPhase::from_text(phase_text) else {
        return Err("unknown phase");
    };

    Ok(SessionUpdate {
        session_id: session_id.to_owned(),
        project_name: project_name.to_owned(),
        phase,
    })
}

/// 持续处理一个客户端的消息，直到断开连接。
fn handle_client(stream: UnixStream, sender: Sender<SessionUpdate>) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut message = String::new();
    loop {
        message.clear();
        if reader.read_line(&mut message)? == 0 {
            return Ok(());
        }
        let update = match parse_session_update(message.trim_end()) {
            Ok(update) => update,
            Err(reason) => {
                let reply = format!("error: {reason}\n");
                reader.get_mut().write_all(reply.as_bytes())?;
                continue;
            }
        };
        sender
            .send_blocking(update)
            .map_err(|err| io::Error::new(io::ErrorKind::BrokenPipe, err))?;
        reader.get_mut().write_all(b"ok\n")?;
    }
}

/// 绑定应用 socket，并在线程中持续接受客户端连接。
pub(crate) fn start_listener(sender: Sender<SessionUpdate>) -> io::Result<()> {
    let listener = UnixListener::bind(constants::SOCKET_PATH)?;
    println!("正在监听：{}", constants::SOCKET_PATH);
    thread::spawn(move || {
        // incoming 持续等待连接，阻塞发生在监听线程里。
        for connection in listener.incoming() {
            let stream = match connection {
                Ok(stream) => stream,
                Err(err) => {
                    eprintln!("socket 接受连接失败：{err}");
                    break;
                }
            };
            let client_sender = sender.clone();
            thread::spawn(move || {
                if let Err(err) = handle_client(stream, client_sender) {
                    eprintln!("socket 客户端通信失败：{err}");
                }
            });
        }
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_session_update;
    use crate::enums::session_phase::SessionPhase;

    #[test]
    fn parses_valid_update() {
        let update = parse_session_update("demo-2|codex-notch|awaiting_approval")
            .expect("合法消息应解析成功");

        assert_eq!(update.session_id, "demo-2");
        assert_eq!(update.project_name, "codex-notch");
        assert_eq!(update.phase, SessionPhase::AwaitingApproval);
    }

    #[test]
    fn rejects_invalid_updates() {
        assert_eq!(
            parse_session_update("demo-2|working").err(),
            Some("invalid format")
        );
        assert_eq!(
            parse_session_update("|codex-notch|working").err(),
            Some("empty session id")
        );
        assert_eq!(
            parse_session_update("demo-2||working").err(),
            Some("empty project name")
        );
        assert_eq!(
            parse_session_update("demo-2|codex-notch|unknown").err(),
            Some("unknown phase")
        );
    }
}
