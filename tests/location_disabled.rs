#[cfg(feature = "track-caller")]
#[test]
fn disabled() {
    use colorz_eyre::eyre;
    use eyre::eyre;

    colorz_eyre::config::HookBuilder::default()
        .display_location_section(false)
        .install()
        .unwrap();

    let report = eyre!("error occured");

    let report = format!("{:?}", report);
    assert!(!report.contains("Location:"));
}
