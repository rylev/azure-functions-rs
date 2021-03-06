use azure_functions::bindings::{HttpRequest, HttpResponse};
use azure_functions::func;
use azure_functions::http::Status;
use serde_json::to_value;

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[func]
#[binding(name = "req", auth_level = "anonymous")]
pub fn greet_with_json(req: &HttpRequest) -> HttpResponse {
    if let Ok(request) = req.body().as_json::<Request>() {
        let response = Response {
            message: format!("Hello from Rust, {}!", request.name),
        };
        return to_value(response).unwrap().into();
    }

    HttpResponse::build()
        .status(Status::BadRequest)
        .body("Invalid JSON request.")
        .into()
}
