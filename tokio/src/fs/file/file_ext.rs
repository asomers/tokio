//#[derive(Debug)]
//#[must_use = "futures do nothing unless polled"]
//pub struct WriteAt<T: T::WriteAt: Future<Output = io::Result<usize>>>(T);

//impl<T: Future<Output = io::Result<usize>>> Future for WriteAt<T> {
    //type Output = io::Result<usize>;

    //fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        //self.0.poll(cx)
    //}

//}


// NB: the Box<> would be unnecessary if these methods were inherent methods of
// File, rather than extension traits.
/// Asynchronous versions of the stuff in [`std::os::unix::fs::FileExt`].
pub trait FileExt {
    /// Writes a number of bytes starting from a given offset.
    ///
    /// Returns the number of bytes written.
    ///
    /// The offset is relative to the start of the file and thus independent
    /// from the current cursor.
    ///
    /// The current file cursor is not affected by this function.
    ///
    /// When writing beyond the end of the file, the file is appropriately
    /// extended and the intermediate bytes are initialized with the value 0.
    ///
    /// Note that similar to File::write, it is not an error to return a short
    /// write.
    ///
    /// Note that since this function does not affect the File's seek position,
    /// multiple calls may be issued concurrently for the same file.
    fn write_at<'a>(&'a self, buf: &'a [u8], ofs: u64) -> super::os::WriteAt<'a>;
    //fn write_at<'a>(&self, buf: &'a mut [u8], ofs: u64)
        //-> Pin<Box<dyn Future<Output = io::Result<usize>> + 'a>>;
}

//impl FileExt for File {
    //fn write_at<'a>(&self, buf: &'a mut [u8], ofs: u64) -> WriteAt<'a, Self>
    //{
        //os::write_at(self, buf, ofs)
    //}

//}
