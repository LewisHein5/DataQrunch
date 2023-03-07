use std::collections::HashMap;
use std::ops::Deref;
use super::dataset_dto::DatasetDto;
use super::session_key::SessionKey;
use super::dataset::Dataset;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Serialize;
use super::user_session_data_cache::UserSessionDataCache;

const ACCESS_DENIED_MESSAGE: &str = "Access Denied";

pub(crate) async fn new_dataset(data: web::Json<DatasetDto>, user_data_cache: web::Data<UserSessionDataCache>) -> impl Responder {
    let access_denied = HttpResponse::Forbidden().body(ACCESS_DENIED_MESSAGE);

    let session_key = match SessionKey::try_from(&data.session_key) {
        Ok(k) => k,
        Err(_) => {return access_denied}
    };

    if !user_data_cache.session_key_is_valid(&session_key) {
        return access_denied;
    }

    let user_data = match user_data_cache.get_user(&session_key) {
        Some(val) => val.deref().clone(),
        None => {return access_denied}
    };

    let dataset = Dataset::from(data);
    match dataset.try_save_to("/tmp/gablorpppp.mp".to_string()) {
        Ok(_) => (),
        Err(e) => {return HttpResponse::InternalServerError().body(e.to_string())} //TODO: This could expose sensitive file names
    }

    //Todo: Make this its own get method
    let dataset = match Dataset::load_from("/tmp/gablorpppp.mp".to_string()) {
        Ok(val) => val,
        Err(e) => {return HttpResponse::InternalServerError().body(e.to_string())} //Todo: This could expose sensitive file names
    };


    let dataset_json = match dataset.to_json() {
        Ok(val) => val,
        Err(e) => {return HttpResponse::InternalServerError().body(e.to_string())} //Todo: This could expose sensitive data
    };

    return HttpResponse::Ok().body(dataset_json); //Todo: need to set a header here
}