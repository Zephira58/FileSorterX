use crate::get_subdir_by_extension;
use std::path::PathBuf;

#[test]
#[rustfmt::skip]
fn test_match_extension() {
    assert_eq!(get_subdir_by_extension("jpg", 1, false), PathBuf::from("image"));
    assert_eq!(get_subdir_by_extension("jpg", 2, false), PathBuf::from("image/gif"));
    assert_eq!(get_subdir_by_extension("jpg", 3, false), PathBuf::from("image/gif"));

    assert_eq!(get_subdir_by_extension("jpg", 1, true), PathBuf::from("image"));
    assert_eq!(get_subdir_by_extension("jpg", 2, true), PathBuf::from("image/animated"));
    assert_eq!(get_subdir_by_extension("jpg", 3, true), PathBuf::from("image/animated/gif"));

    assert_eq!(get_subdir_by_extension("qt", 1, false), PathBuf::from("video"));
    assert_eq!(get_subdir_by_extension("qt", 2, false), PathBuf::from("video/quicktime"));
    assert_eq!(get_subdir_by_extension("qt", 3, false), PathBuf::from("video/quicktime"));

    assert_eq!(get_subdir_by_extension("qt", 1, true), PathBuf::from("video"));
    assert_eq!(get_subdir_by_extension("qt", 2, true), PathBuf::from("video/quicktime"));
    assert_eq!(get_subdir_by_extension("qt", 3, true), PathBuf::from("video/quicktime"));

    assert_eq!(get_subdir_by_extension("mp4", 1, false), PathBuf::from("video"));
    assert_eq!(get_subdir_by_extension("mp4", 2, false), PathBuf::from("video/mp4"));
    assert_eq!(get_subdir_by_extension("mp4", 3, false), PathBuf::from("video/mp4"));

    assert_eq!(get_subdir_by_extension("mp4", 1, true), PathBuf::from("video"));
    assert_eq!(get_subdir_by_extension("mp4", 2, true), PathBuf::from("video/mp4"));
    assert_eq!(get_subdir_by_extension("mp4", 3, true), PathBuf::from("video/mp4"));
}

#[test]
#[should_panic(expected = "Nesting level is out of range.")]
fn test_match_extension_panic_less() {
    get_subdir_by_extension("s", 0, false);
}

#[test]
#[should_panic(expected = "Nesting level is out of range.")]
fn test_match_extension_panic_more() {
    get_subdir_by_extension("s", 4, false);
}
