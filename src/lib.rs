pub use tracing;

// re-export for scoped_logger! macro
pub use actix_web;
pub use futures;
pub use uuid;

use tracing::{event, Level};

use std::env::{self, VarError};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
  INIT.call_once(|| {
    tracing_subscriber::fmt()
      .with_ansi(false)
      .without_time()
      .with_env_filter(
        tracing_subscriber::EnvFilter::from_default_env(),
      )
      .init();
  });
}

pub fn log_simple_err_callback<E: std::fmt::Debug, M: AsRef<str>>(
  msg: M,
) -> impl FnOnce(E) -> E {
  move |e| {
    log_simple_err(msg, &e);
    e
  }
}

pub fn log_simple_err<E: std::fmt::Debug, M: AsRef<str>>(
  msg: M,
  err: &E,
) {
  event!(Level::ERROR, msg = msg.as_ref(), error = ?err);
}

pub fn logged_var(variable_name: &str) -> Result<String, VarError> {
  env::var(variable_name).map_err(|e| {
    event!(
      Level::ERROR,
      msg = "unset environment",
      variable = variable_name
    );
    VarError::from(e)
  })
}

#[macro_export]
macro_rules! scoped_logger {
  () => {
    |mut req, srv| {
      use jonases_tracing_util::actix_web::dev::Service;
      use jonases_tracing_util::actix_web::http::{HeaderName, HeaderValue};

      let mut headers = req.headers_mut();

      let request_id = if let Some(id) = headers.get("x-request-id") {
        (&*String::from_utf8_lossy(id.as_bytes())).to_owned()
      } else {
        let request_id =
          jonases_tracing_util::uuid::Uuid::new_v4().to_string();

        // savely unwrapable, because I know what data I pass
        let name =
          HeaderName::from_lowercase(b"x-request-id").unwrap();
        let value = HeaderValue::from_str(&request_id).unwrap();

        headers.insert(name, value);

        request_id
      };

      let uri = req.uri().clone();
      let res = srv.call(req);

      async move {
        let span = jonases_tracing_util::tracing::span!(
          jonases_tracing_util::tracing::Level::INFO,
          "span",
          %uri,
          %request_id
        );
        let _enter = span.enter();

        match res.await {
          Ok(mut res) => {
            let status = res.status();

            if !res.status().is_success() {
              jonases_tracing_util::tracing::event!(
                jonases_tracing_util::tracing::Level::ERROR,
                msg = "unsuccessful response",
                status = %status,
              );
            } else {
              jonases_tracing_util::tracing::event!(
                jonases_tracing_util::tracing::Level::INFO,
                msg = "successful response",
                status = %status,
              );
            }
            Ok(res)
          },
          Err(e) => {
            let status = e.as_response_error().status_code();

            jonases_tracing_util::tracing::event!(
              jonases_tracing_util::tracing::Level::ERROR,
              msg = "unsuccessful response",
              status = %status,
              error_body = %e,
            );

            Err(e)
          },
        }
      }
    }
  }
}
