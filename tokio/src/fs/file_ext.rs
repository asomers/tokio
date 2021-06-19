pub trait FileExt {
    pub fn write_at<'a>(&self, buf: &'a mut [u8], ofs: u64) -> WriteAt<'a, Self>
    {
        let std = self.std.clone();
        asyncify(move || std.write_at(buf, offset))
    }
}
