use super::super::super::*;

pub(in super::super::super) async fn sync_all(file: &File) -> io::Result<()> {
    super::generic::sync_all(file).await
}

pub(in super::super::super) async fn write_at<'a>(
    file: &'a File,
    buf: &'a mut [u8],
    ofs: u64
) -> io::Result<usize>
{
    super::generic::write_at(file, buf, ofs).await
}
