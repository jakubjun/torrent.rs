use futures::future::join_all;
use std::{
    io::{self, Cursor, SeekFrom},
    sync::Arc,
};

use tokio::{
    fs::File,
    io::{AsyncSeekExt, AsyncWrite, AsyncWriteExt},
    sync::Mutex,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = Arc::new(Mutex::new(File::create("foo.txt").await?));
    let mut handles = vec![];
    for i in 1..10 {
        let mut file = Arc::clone(&file);
        let handle = tokio::spawn(async move {
            let mut file = file.lock().await;

            // file.seek(SeekFrom::Start(i * 12)).await;
            file.write_all(format!("hello from {}\n", i).as_bytes())
                .await;
        });
        handles.push(handle);
    }
    futures::future::join_all(handles).await;
    Ok(())
}
