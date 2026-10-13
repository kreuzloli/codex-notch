use std::{
    io::{self, BufRead, BufReader, Read, Write},
    os::unix::net::UnixStream,
    path::Path,
    time::Duration,
};

use serde::Deserialize;

/// 只提取转发状态所需的 Hook 字段。
#[derive(Deserialize)]
struct HookEvent {
    session_id: String,
    cwd: String,
    hook_event_name: String,
}

/// 读取 Codex Hook 通过标准输入传入的事件。
fn main() -> io::Result<()> {
    eprintln!("hook-relay start");
    let mut input = String::new();
    // 一直读取到输入结束，将内容追加到 input。
    io::stdin().read_to_string(&mut input)?;
    // 将 JSON 解析错误转换为当前函数使用的 I/O 错误类型。
    let event = serde_json::from_str::<HookEvent>(&input)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    eprintln!(
        "Session：{}，current working directory：{}，event：{}",
        event.session_id, event.cwd, event.hook_event_name
    );
    // 将Codex 事件转换为现有socket协议中的状态
    let phase = match event.hook_event_name.as_str() {
        "UserPromptSubmit" => "working",
        "Stop" => "finished",
        _ => return Ok(()),
    };
    eprintln!("event：{}，phase：{phase}", event.hook_event_name);
    // 取工作目录最后一级名称，用于卡片展示。
    let project_name = Path::new(&event.cwd)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "工作目录没有有效的项目名"))?;
    // 项目名不能包含当前行协议的分隔符。
    if project_name.contains(['|', '\n', '\r']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "项目名包含协议分隔符",
        ));
    }
    let message = format!("{}|{}|{}\n", event.session_id, project_name, phase);
    eprintln!("待发送消息：{}", message.trim_end());
    // 链接灵动岛，并限制通信等待时间
    let mut stream = UnixStream::connect("/tmp/codex-notch.sock")?;
    let timeout = Some(Duration::from_secs(1));
    stream.set_read_timeout(timeout)?;
    stream.set_write_timeout(timeout)?;
    // 将完整的一行消息发送给服务端。
    stream.write_all(message.as_bytes())?;
    // 服务端保持连接，所以按换行读取回复，不等待连接关闭。
    let mut reader = BufReader::new(stream);
    let mut reply = String::new();
    reader.read_line(&mut reply)?;
    if reply.trim_end().to_ascii_lowercase() != "ok" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Status forward failed：{}", reply.trim_end()),
        ));
    }
    eprintln!("Status has been forwarded.");
    // Stop Hook 成功时向 Codex 返回一个空 JSON 对象。
    if event.hook_event_name == "Stop" {
        println!("{{}}");
    }
    Ok(())
}
