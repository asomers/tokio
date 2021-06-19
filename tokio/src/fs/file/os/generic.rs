//! OS-independent implementations of asynchronous filesystem functions
use super::super::super::*;

#[cfg(unix)]
use std::{
    future::Future,
    os::unix::fs::FileExt,
    pin::Pin,
    task::{Context, Poll},
};

pub(in crate::fs) async fn sync_all(file: &File) -> io::Result<()> {
    let mut inner = file.inner.lock().await;
    inner.complete_inflight().await;

    let std = file.std.clone();
    asyncify(move || std.sync_all()).await
}

#[cfg(unix)]
pub(in crate::fs) fn write_at<'a>(file: &'a File, buf: &'a [u8], ofs: u64) -> WriteAt<'a>
{
    let state = WriteAtState::Idle;
    WriteAt {state, file, buf, ofs}
}

#[derive(Debug)]
enum WriteAtState {
    Idle,
    Busy(sys::Blocking<io::Result<usize>>),
}

#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub(in crate::fs) struct WriteAt<'a> {
    state: WriteAtState,
    file: &'a File,
    buf: &'a [u8],
    ofs: u64
}

impl<'a> Future for WriteAt<'a> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            WriteAtState::Idle => {
                let buf = Vec::from(self.buf);
                let ofs = self.ofs;
                let file = self.file.std.clone();

                self.state = WriteAtState::Busy(sys::run(move || {
                    let res = file.write_at(&buf[..], ofs);
                    res
                }));

                Poll::Pending
            }
            WriteAtState::Busy(ref mut rx) => {
                let res = ready!(Pin::new(rx).poll(cx))?;
                self.state = WriteAtState::Idle;
                Poll::Ready(res)
            }
        }
    }
}

