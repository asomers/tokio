//! Portable POSIX implementation of FileExt
use super::super::super::*;

#[cfg(unix)]
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

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
    fn write_at<'a>(&'a self, buf: &'a mut [u8], ofs: u64) -> WriteAt<'a>
    {
        WriteAt(super::generic::write_at(self, buf, ofs))
    }
}
