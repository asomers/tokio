//! FreeBSD implementation of FileExt
// TODO: replace the generic stuff with POSIX AIO
use super::super::super::*;

use mio_aio::{AioCb, AioFsyncMode};
use std::{
    future::Future,
    os::unix::io::AsRawFd,
    pin::Pin,
    task::{Context, Poll},
};
use crate::io::PollAio;

/// Represents the state of a single AIO operation
#[derive(Debug, Eq, PartialEq)]
enum AioState {
    /// The AIO operation has no in-kernel state
    Idle,
    /// The kernel is processing the AIO operation
    Busy
}

/// Future for the [`write_at`](crate::fs::file::FileExt::write_at) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct WriteAt<'a>(super::generic::WriteAt<'a>);

impl<'a> Future for WriteAt<'a> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

impl super::super::FileExt for File {

    fn write_at<'a>(&'a self, buf: &'a [u8], ofs: u64) -> WriteAt<'a>
    {
        WriteAt(super::generic::write_at(self, buf, ofs))
    }
}

/// Future for the [`sync_all`](crate::fs::file::sync_all) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct SyncAll {
    state: AioState,
    op: PollAio<AioCb<'static>>
}

impl Future for SyncAll {
    type Output = io::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.op.poll(cx) {
            Poll::Pending => {
                if self.state == AioState::Idle {
                    let r = (*self.op).fsync(AioFsyncMode::O_SYNC);
                    if let Err(e) = r {
                        return Poll::Ready(Err(e.into()));
                    }
                    self.state = AioState::Busy;
                }
                Poll::Pending
            },
            Poll::Ready(Ok(_ev)) => {
                let result = (*self.op).aio_return().map(drop);
                Poll::Ready(result)
            },
            Poll::Ready(Err(e)) => Poll::Ready(Err(e))
        }
    }
}

pub(in super::super) async fn sync_all(file: &File) -> io::Result<()> {
    let aiocb = AioCb::from_fd(file.std.as_raw_fd(), 0);
    let pe = PollAio::new_for_aio(aiocb)?;
    SyncAll{
        state: AioState::Idle,
        op: pe
    }.await
    // TODO: handle EAGAIN, EOPNOTSUPP, and EINVAL by
    // falling back to the generic implementation.
}
