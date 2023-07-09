
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        use uuid::Uuid;
            let error_uuid: Uuid = uuid::Uuid::new_v4();
            log::error!("Error ID: {}, error condition: {}", error_uuid, format!($($arg)*));
            format!("{{\"Error ID\": \"{}\"}}", error_uuid)
        }}
}
