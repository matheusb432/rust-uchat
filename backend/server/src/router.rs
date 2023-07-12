use crate::{
    handler::{with_handler, with_public_handler},
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Method};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use uchat_endpoint::{
    post::endpoint::{Bookmark, NewPost, React, TrendingPosts},
    user::endpoint::{CreateUser, Login},
    Endpoint,
};

pub fn new_router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/", get(move || async { "this is the root page" }))
        .route(CreateUser::URL, post(with_public_handler::<CreateUser>))
        .route(Login::URL, post(with_public_handler::<Login>));
    let authorized_routes = Router::new()
        .route(NewPost::URL, post(with_handler::<NewPost>))
        .route(TrendingPosts::URL, post(with_handler::<TrendingPosts>))
        .route(Bookmark::URL, post(with_handler::<Bookmark>))
        .route(React::URL, post(with_handler::<React>));

    Router::new()
        .merge(public_routes)
        .merge(authorized_routes)
        .layer(
            // NOTE With ServiceBuilder, the added layers will run in the order in which they are added
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Micros),
                        ),
                )
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                        .allow_credentials(true)
                        .allow_origin(
                            std::env::var("FRONTEND_URL")
                                .unwrap()
                                .parse::<HeaderValue>()
                                .unwrap(),
                        )
                        .allow_headers([CONTENT_TYPE]),
                )
                // ? Attaching the application state to the layers
                .layer(axum::Extension(state.clone())),
        )
        // ? Attaching the application state to the routes
        .with_state(state)
}
