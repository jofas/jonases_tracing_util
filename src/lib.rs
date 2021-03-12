pub use tracing;

// re-export for scoped_logger! macro
pub use actix_web;
pub use uuid;
pub use futures;

use tracing::{event, Level};

use std::env::{self, VarError};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
  INIT.call_once(|| {
    tracing_subscriber::fmt()
      .with_ansi(false)
      .with_env_filter(
        tracing_subscriber::EnvFilter::from_default_env(),
      )
      .init();
  });
}

pub fn log_simple_err_callback<E: std::fmt::Debug>(
  msg: &'static str,
) -> impl FnOnce(E) -> E {
  move |e| {
    log_simple_err(msg, &e);
    e
  }
}

pub fn log_simple_err<E: std::fmt::Debug>(msg: &str, err: &E) {
  event!(Level::ERROR, msg, error = ?err);
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

// I'm too lazy to use proper actix types, they totally overdid it
#[macro_export]
macro_rules! scoped_logger {
  () => {
    |req, srv| {
      use jonases_tracing_util::futures::stream::StreamExt;

      let request_id = jonases_tracing_util::uuid::Uuid::new_v4();
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
            if !res.status().is_success() {
              res = res.map_body(|_, b| {
                let buf =
                  jonases_tracing_util::actix_web::web::BytesMut::new();

                let body = b.fold(buf, |mut acc, x| {
                  acc.extend_from_slice(&*x);
                  acc
                }).await;

                jonases_tracing_util::log_simple_err("unsuccessful response", &err);

                /*
                if let Some(body) = b.as_ref() {
                  if let jonases_tracing_util::actix_web::dev::Body::Bytes(bytes) = body {
                    let err = String::from_utf8_lossy(bytes);
                  } else {
                    let err = format!("{:?}", body);
                    jonases_tracing_util::log_simple_err("unsuccessful response", &err);
                  }
                } else {
                  let err = "no response body";
                  jonases_tracing_util::log_simple_err("unsuccessful response", &err);
                }
                b
                */
                jonases_tracing_util::actix_web::dev::Body::Bytes(&*body)
              });
            }
            jonases_tracing_util::tracing::event!(
              jonases_tracing_util::tracing::Level::INFO,
              "successful response"
            );
            Ok(res)
          }
          Err(e) => {
            jonases_tracing_util::log_simple_err("unsuccessful response", &e);
            Err(e)
          }
        }
      }
    }
  }
}
