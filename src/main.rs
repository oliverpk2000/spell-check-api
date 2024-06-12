use axum::extract::Path;
use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "the api is at /words/:word" }))
        .route("/words/:word", get(get_corrected_words));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_corrected_words(Path(arg): Path<String>) -> Json<Vec<String>> {
    let correct_words = vec![arg, "test".parse().unwrap(), "test2".parse().unwrap()];
    Json(correct_words)
}