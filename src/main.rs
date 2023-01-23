mod endpoints;
mod env_data;
use actix_web::{web::Data, App, HttpServer};
use endpoints::{read, store_data};
use env_data::EnvDataEntry;
use mongodb::{options::ClientOptions, Client};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "home_enviorment_monitor",
    about = "Home environment monitoring"
)]
struct Opt {
    #[structopt(short = "h", long = "host", default_value = "0.0.0.0:65534")]
    host: String,

    #[structopt(default_value = "65534")]
    port: u16,

    #[structopt(
        short = "d",
        long = "db_url",
        default_value = "mongodb://localhost:27017"
    )]
    db_url: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //parse args
    let opt = Opt::from_args();

    let client_options = ClientOptions::parse(&opt.db_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("home_enviorment_monitor");
    let collection = db.collection::<EnvDataEntry>("data");

    //configure and start the server
    HttpServer::new(move || {
        App::new()
            .service(store_data)
            .service(read)
            .app_data(Data::new(collection.clone()))
    })
    .bind(format!("{}:{}", opt.host, opt.port))?
    .run()
    .await
    .unwrap();
    Ok(())
}
