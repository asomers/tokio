//! Portable POSIX implementation of FileExt
use super::super::super::*;

#[cfg(unix)]
use std::{
    future::Future,
    os::unix::fs::FileExt,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
enum WriteAtState {
    Idle,
    Busy(sys::Blocking<io::Result<usize>>),
}

pub(super) struct WriteAt<'a> {
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

impl super::super::FileExt for File {
    type WriteAt = WriteAt<'_>;

    fn write_at<'a>(&self, buf: &'a mut [u8], ofs: u64) -> Self::WriteAt<'a> {
        let state = WriteAtState::Idle;
        WriteAt {state, file, buf, ofs}
    }
}


