use cfg_if::cfg_if;

// TODO: not public
pub(super) mod generic;

cfg_if! {
    if #[cfg(target_os = "freebsd")] {
        mod freebsd;
        pub(super) use self::freebsd::WriteAt;
    } else if #[cfg(unix)] {
        mod unix;
        pub(super) use self::unix::*;
    }
}

cfg_if! {
    if #[cfg(target_os = "freebsd")] {
        pub(super) use self::freebsd::sync_all;
    } else {
        pub(super) use self::generic::sync_all;
    }
}
