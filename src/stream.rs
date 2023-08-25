//! Utils to deal with streams data types.

use actix_http::error::PayloadError;
use actix_web::web::Bytes;
use awc::ResponseBody;
use futures_core::stream::Stream;
use crate::result::{AppError, Result};

/// Read body from an HTTP response as string.
/// The content has to be encoded in UTF-8, otherwise
/// [`AppError::Unexpected`] is returned.
/// # Example
/// ```example
/// use actix_contrib_rest::stream::read_body;
/// use awc::Client;
///
/// let client = Client::default();
/// let mut res = client.get("http://example.com/")
///             .send()
///             .await
///             .unwrap_or_else(|e| {
///                 eprintln!("{}", e);
///             });
/// println!("Response: {}", read_body(res.body()).await?);
/// ```
pub async fn read_body<S>(body: ResponseBody<S>) -> Result<String>
where
    S: Stream<Item = core::result::Result<Bytes, PayloadError>>,
{
    let bytes = body.await.unwrap().to_vec();
    String::from_utf8(bytes).map_err(|e| AppError::Unexpected(e.into()))
}
