use std::str::FromStr;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    payload::PlainText,
    payload::Json,
    OpenApi,
    OpenApiService,
    param::Query,
    Object
};

use question_generator::{
    Question,
    GeneratorParameters,
    generator::maths::{ArithmeticOperation, MathsGeneratorParameters},
    generator::maths::generate as generate_maths
};

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
//
// TODO - this is going to get cumbersome with the number of parameters we
// could end up with just by combining all the fields from all the generators,
// so we might be better off having a POST endpoint that can accept a JSON
// object containing all the generator params.
#[OpenApi]
impl QuestionsApi {
    /// Hello world
    #[oai(path = "/questions", method = "get")]
    async fn index(&self,
        subject: Query<Option<String>>,
        count: Query<Option<usize>>,
        answer_count: Query<Option<usize>>,
        operations: Query<String>
    ) -> Json<Vec<Question>> {
        Json(generate_maths(
            GeneratorParameters {
                count: count.unwrap_or(3),
                answer_count: answer_count.unwrap_or(3)
            },
            MathsGeneratorParameters {
                operations:
                    operations.split(',').collect::<Vec<_>>().iter().map(|o| ArithmeticOperation::from_str(o).unwrap()).collect()
            }
        ))
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
