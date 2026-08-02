// The shared file-size gate from CODE_STANDARDS §2.2 — the 800-line hard
// limit on non-test lines — enforced under plain `cargo test`. The
// `kaiju_server` sibling crate lives in this repo, so it is gated from here
// too.

#[test]
fn source_files_stay_under_the_limit() {
    macroquad_toolkit::source_gate::assert_source_files_within_limit(
        env!("CARGO_MANIFEST_DIR"),
        &[],
    );
}

#[test]
fn server_source_files_stay_under_the_limit() {
    macroquad_toolkit::source_gate::assert_source_files_within_limit(
        concat!(env!("CARGO_MANIFEST_DIR"), "/kaiju_server"),
        &[],
    );
}
