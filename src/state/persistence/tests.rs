use super::*;

#[test]
fn test_save_path() {
    let path = get_save_path();
    assert!(path.is_ok());
}
