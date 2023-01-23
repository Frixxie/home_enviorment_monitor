use actix_web::{
    get, post,
    web::{self, Data, Json},
    Responder,
};
use futures::TryStreamExt;
use mongodb::Collection;

use crate::env_data::{EnvData, EnvDataEntry};

#[get("/")]
async fn read(
    collection: Data<Collection<EnvDataEntry>>,
) -> Result<impl Responder, actix_web::Error> {
    let result = collection.find(None, None).await;
    match result {
        Ok(cursor) => {
            let entries = cursor.try_collect::<Vec<EnvDataEntry>>().await;
            match entries {
                Ok(entries) => Ok(Json(
                    entries.into_iter().max_by_key(|e| e.timestamp).unwrap(),
                )),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
            }
        }
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[post("/")]
async fn store_data(
    collection: Data<Collection<EnvDataEntry>>,
    data: web::Json<EnvData>,
) -> Result<impl Responder, actix_web::Error> {
    let data_entry: EnvDataEntry = data.into_inner().into();
    let result = collection.insert_one(data_entry, None).await;
    match result {
        Ok(_) => Ok("OK".to_string()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
