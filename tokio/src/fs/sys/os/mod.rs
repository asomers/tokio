#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(target_os = "freebsd")]
pub(in super::super) use self::freebsd::*;

mod generic;
