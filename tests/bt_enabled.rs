use colorz_eyre::eyre;
use eyre::eyre;

#[test]
fn enabled() {
    colorz_eyre::config::HookBuilder::default()
        .display_env_section(true)
        .install()
        .unwrap();

    let report = eyre!("error occured");

    let report = format!("{:?}", report);
    assert!(report.contains("RUST_BACKTRACE"));
}
