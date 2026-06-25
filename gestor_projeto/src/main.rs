use axum::{
    routing::get,
    Router
};

async fn health() -> &'static str {
    "Gestão de Projetos - API"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3000"
        ).await
         .unwrap();

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
