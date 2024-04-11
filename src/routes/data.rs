use rocket::form::FromForm;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, FromForm)]
struct DataReq {
    post_id: u64,
    title: String,
    user_id: String
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct DataRes {
    summary: String
}

#[openapi(tag = "Data")]
#[get("/data?<data_req..>")]
pub fn get_data(data_req: DataReq) -> Option<Json<DataRes>> {
    Some(Json(DataRes{
        summary: "TEST".to_owned()
    }))
}