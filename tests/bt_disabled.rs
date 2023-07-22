use colorz_eyre::eyre;
use eyre::eyre;

#[test]
fn disabled() {
    colorz_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()
        .unwrap();

    let report = eyre!("error occured");

    let report = format!("{:?}", report);
    assert!(!report.contains("RUST_BACKTRACE"));
}
