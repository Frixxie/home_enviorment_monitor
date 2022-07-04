use actix_web::{get, post, web, App, Either, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::fmt;
use std::time::SystemTime;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "home_enviorment_monitor",
    about = "Home environment monitoring"
)]
struct Opt {
    #[structopt(
        short = "l",
        long = "listen_url",
        default_value = "192.168.0.167:65534"
    )]
    listen_url: String,

    #[structopt(short = "u", long = "db_url", default_value = "")]
    db_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, sqlx::FromRow)]
pub struct EnvData {
    pub room: String,
    pub temperature: f32,
    pub humidity: f32,
}

impl EnvData {
    pub fn new(room: String, temperature: f32, humidity: f32) -> Self {
        Self {
            room,
            temperature,
            humidity,
        }
    }
}

impl fmt::Display for EnvData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.room,
            (self.temperature as f32),
            (self.humidity as f32),
        )
    }
}

#[get("/")]
async fn read(pool: web::Data<PgPool>) -> Either<impl Responder, impl Responder> {
    let rows = sqlx::query_as::<_, EnvData>(
        "SELECT room, temperature, humidity FROM env_data ORDER BY time DESC LIMIT 1",
    )
    .fetch_one(&**pool)
    .await;

    match rows {
        Ok(row) => Either::Left(web::Json(row)),
        Err(e) => {
            println!("{:?}", e);
            Either::Right(HttpResponse::InternalServerError())
        }
    }
}

#[post("/")]
async fn index(data: web::Json<EnvData>, pool: web::Data<PgPool>) -> String {
    println!("{}", data);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    sqlx::query("CREATE TABLE IF NOT EXISTS home_env (time INT, room TEXT, temp REAL, hum REAL)")
        .execute(&**pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO home_env (time, room, temp, hum) VALUES ($1, $2, $3, $4)")
        .bind(now as i64)
        .bind(data.room.as_str())
        .bind(data.temperature)
        .bind(data.humidity as i16)
        .execute(&**pool)
        .await
        .unwrap();
    "Ok".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let pool = web::Data::new(
        PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(5))
            .connect(&opt.db_url)
            .await
            .unwrap(),
    );
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(read)
            .app_data(pool.clone())
    })
    .bind(opt.listen_url)?
    .run()
    .await
    .unwrap();
    Ok(())
}
