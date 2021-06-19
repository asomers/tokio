use cfg_if::cfg_if;

// TODO: not public
pub(super) mod generic;

cfg_if! {
    if #[cfg(target_os = "freebsd")] {
        mod freebsd;
        pub(super) use self::freebsd::*;
    } else {
        mod unix;
        pub(super) use self::unix::*;
    }
}
