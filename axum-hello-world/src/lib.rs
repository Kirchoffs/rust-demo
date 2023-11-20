mod routes;

use axum::Server;
use routes::create_routes;

pub async fn run() {
    let app = create_routes();

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
