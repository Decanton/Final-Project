use warp::{Filter, http::StatusCode, reply::{self, with_status}, Rejection};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Deserialize, Serialize)]
struct MemeRequest {
    image: String,
    text: String,
}

#[tokio::main]
async fn main() {
    // Define the route for meme generation
    let generate_meme = warp::post()
        .and(warp::path("generateMeme"))
        .and(warp::body::json())
        .and_then(handle_generate_meme)
        .recover(handle_rejection);

    // Run the server on localhost:3030
    warp::serve(generate_meme)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_generate_meme(request: MemeRequest) -> Result<impl warp::Reply, Infallible> {
    let image_url = request.image;
    let text = request.text;

    // Placeholder: actual meme generation logic goes here
    // For now, just return the received image URL and text
    let meme_url = format!("Image URL: {}, Text: {}", image_url, text);

    Ok(reply::json(&serde_json::json!({ "url": meme_url })))
}

async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    let code = if err.is_not_found() {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let message = if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        format!("Error deserializing request body: {}", e)
    } else {
        "Internal server error".to_string()
    };

    Ok(with_status(message, code))
}