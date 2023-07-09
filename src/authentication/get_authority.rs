pub(crate) fn get_authority() -> String {
    return std::env::var("AUTHORITY").expect("AUTHORITY must be set"); //TODO: Fix panic
}