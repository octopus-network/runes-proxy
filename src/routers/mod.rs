use crate::middleware::cors::cors_middleware;
use salvo::prelude::*;
use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};
use serde::{Deserialize, Serialize};

pub mod runes;
mod static_routers;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
struct Request<T: ToSchema + std::fmt::Debug + 'static> {
    request: T,
}

pub fn router() -> Router {
    let _cors_handler = cors_middleware();
    let mut static_routers = static_routers::create_static_routers();
    let router = Router::new()
        //.hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .push(Router::with_path("r/block").post(runes::block))
        .push(Router::with_path("r/blockcount").post(runes::blockcount))
        .push(Router::with_path("r/blockhash").post(runes::blockhash))
        .push(Router::with_path("r/blockhash_by_height").post(runes::blockhash_by_height))
        .push(Router::with_path("r/blockheight").post(runes::blockheight))
        .push(Router::with_path("r/blocks").post(runes::blocks))
        .push(Router::with_path("r/blocktime").post(runes::blocktime))
        .push(Router::with_path("r/clock").post(runes::clock))
        .push(Router::with_path("r/collections").post(runes::collections))
        .push(Router::with_path("r/inscriptions").post(runes::inscriptions))
        .push(Router::with_path("r/tx").post(runes::tx))
        .push(Router::with_path("r/runes").post(runes::runes))
        .push(Router::with_path("r/rune").post(runes::rune))
        .push(Router::with_path("r/status").post(runes::status))
        .append(&mut static_routers);
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
