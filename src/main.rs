use poem::get;
use poem::post;
use poem::Route;
use poem::Server;
use poem::listener::TcpListener;

mod lib;
mod routes;

use crate::routes::ape_get::ape_get;
use crate::routes::ape_post::ape_post;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/ape_get", get(ape_get))
        .at("/ape_post", post(ape_post))
        ;

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
