#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tempfile::NamedTempFile;
use tokio::fs::{File, FileExt};

const HELLO: &[u8] = b"hello world...";

fn tempfile() -> NamedTempFile {
    NamedTempFile::new().unwrap()
}

#[tokio::test]
async fn basic_write_at() {
    let tempfile = tempfile();

    let file = File::create(tempfile.path()).await.unwrap();

    let len = file.write_at(HELLO, 0).await.unwrap();

    assert_eq!(len, HELLO.len(), "maybe need write_all_at");
    let file = std::fs::read(tempfile.path()).unwrap();
    assert_eq!(file, HELLO);
}


