use cfg_aliases::cfg_aliases;

pub fn make_aliases() {
    cfg_aliases! {
        use_jack: {
            all(
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd"
                ),
                feature = "jack"
            )
        },
    }
}


