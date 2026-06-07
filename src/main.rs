use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    payload::PlainText,
    payload::Json,
    OpenApi,
    OpenApiService,
    param::Query,
    Object
};

#[derive(Object)]
struct Question {
    text: String,
    answers: Vec<String>,
    correct_answer: u32,
    explanation: Option<String>,
}

// TODO - we can put this in a module, so we can keep it tidy when we add the
// metadata API alongside it
struct QuestionsApi;

// How do we associate the subject in the query with the algorithm to generate
// the questions? Different curricula will have different generators: maths
// will mostly be easy to generate purely arithmetic questions, but for worded
// questions will require the database, so somehow we need to control exactly
// which algorithm gets run depending on the query.
//
// For now we only have arithmetic so we can just use that directly...
#[OpenApi]
impl QuestionsApi {
    /// Hello world
    #[oai(path = "/questions", method = "get")]
    async fn index(&self, subject: Query<Option<String>>) -> Json<Question> {
        Json(Question {
            text: "some_text".to_string(),
            answers: vec![ "A".to_string(), "B".to_string(), "C".to_string() ],
            correct_answer: 0,
            explanation: None
        })
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
