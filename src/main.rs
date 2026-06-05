use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService, param::Query};
use serde::Deserialize;

#[derive(Deserialize)]
struct GeneratorQuery {
    subject: String
}

struct QuestionsApi;

#[OpenApi]
impl QuestionsApi {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self, subject: Query<String>) -> PlainText<String> {
        PlainText(format!("Hello World {}", subject.0))
    }
}

#[tokio::main]
async fn main() {
    let api_service =
        OpenApiService::new(QuestionsApi, "Hello World", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
