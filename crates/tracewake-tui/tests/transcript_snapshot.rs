use tracewake_tui::transcript::capture_representative_transcript;

#[test]
fn transcript_snapshot_is_byte_identical_across_runs() {
    let first = capture_representative_transcript().unwrap();
    let second = capture_representative_transcript().unwrap();

    assert_eq!(first.as_bytes(), second.as_bytes());
    assert!(first.contains("== view.initial =="));
    assert!(first.contains("Why-not:"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Event Log"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Replay"));
}
