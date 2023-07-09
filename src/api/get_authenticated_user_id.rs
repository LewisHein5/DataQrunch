use actix_web::HttpRequest;

pub (super) fn get_authenticated_user_id(req: HttpRequest) -> String {
    req
        .headers()
        .get("UserId")
        .expect("Grr")
        .to_str()
        .expect("grr")
        .to_string() //TODO: don't panic
}