pub(super) fn get_user_dir_name(user_id: &String) -> String {
    user_id
        .replace("@", "_at_")
        .replace("\"", "")
}