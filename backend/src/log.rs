use crate::db::{add_line, ReadLineError};
use crate::models::Log;
use crate::MyConn;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::broadcast::Sender;

pub async fn monitor(file: PathBuf, conn: MyConn, channel: Sender<Log>) {
    let mut child = match Command::new("tail")
        .arg("-F")
        .arg("-n")
        .arg("+1")
        .arg(&file)
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(command) => command,
        Err(e) => panic!(
            "Error running tail command on file {}\n{}\n",
            &file.display(),
            e,
        ),
    };

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("tail status was: {}", status);
        panic!("Tail died");
    });

    while let Some(line) = reader.next_line().await.unwrap() {
        match add_line(&conn, line).await {
            Ok(result) => {
                let _res = channel.send(result).ok();
            }
            Err(error) => match error {
                ReadLineError::LogExists(_log) => (),
                ReadLineError::ParseError(line, context) => println!("Error {context}\n{line}"),
            },
        }
    }
}
