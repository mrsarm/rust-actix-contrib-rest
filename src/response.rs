//! Handlers to manage HTTP responses.

use crate::result::ValidationErrorPayload;

use actix_web::error::InternalError;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_validator::Error;

/// Function to handle validation errors when serializing the request payload (JSON body),
/// or the query string, generating a HTTP 400 error with a JSON body
/// describing the error. It has to be configured with the [`JsonConfig`](https://docs.rs/actix-web-validator/latest/actix_web_validator/struct.JsonConfig.html)
/// extractor from the [actix-web-validator](https://docs.rs/actix-web-validator) validator crate.
/// # Example
/// Configure the method as follow:
/// ```example
/// use actix_web::{web, App};
/// use actix_web_validator::JsonConfig;
/// use actix_contrib_rest::response::json_error_handler;
///
/// fn main() {
///     let app = App::new().service(
///         web::resource("/api")
///             // ...
///             .app_data(JsonConfig::default().error_handler(json_error_handler))
///             .route(web::post().to(api_method))
///     );
/// }
/// ```
/// If there is an error in the validations, the response will look like:
/// ```json
/// {
///   "error": "Validation error",
///   "field_errors": {
///     "name": [
///       {
///         "code": "length",
///         "message": null,
///         "params": {
///           "max": 50,
///           "min": 3,
///           "value": "Bi"
///         }
///       }
///     ]
///   }
/// }
/// ```
pub fn json_error_handler(err: Error, _req: &HttpRequest) -> actix_web::error::Error {
    let json_error = match &err {
        Error::Validate(error) =>
            HttpResponse::BadRequest().json(ValidationErrorPayload::from(error)),
        Error::JsonPayloadError(error) =>
            HttpResponse::UnprocessableEntity()
                .json(ValidationErrorPayload::new(error.to_string())),
        _ =>
            HttpResponse::BadRequest()
                .json(ValidationErrorPayload::new(err.to_string())),
    };
    InternalError::from_response(err, json_error).into()
}
