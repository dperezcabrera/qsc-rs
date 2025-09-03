pub fn allowed_sig_algs() -> Vec<String> {
    std::env::var("QSC_SIG_ALGS")
        .unwrap_or_else(|_| "mldsa3".into())
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}
