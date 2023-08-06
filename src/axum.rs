use ::axum::{
  response::{IntoResponse, Response},
  http::HeaderValue,
  Json,
};

/// Similar to [`axum::Json`], but gives a `Content-Type` header that matches
/// ActivityStreams instead of a generic JSON content type.
#[derive(Clone, Copy, Debug)]
pub struct ActivityJson<T>(pub T);

impl<T> IntoResponse for ActivityJson<T>
where
  Json<T>: IntoResponse,
{
  fn into_response(self) -> Response {
      let json = Json(self.0);
      let mut response = json.into_response();
      response.headers_mut().insert(
          "content-type",
          HeaderValue::from_static("application/activity+json"),
      );
      response
  }
}
