use tracing::{event, Level};

use std::env::{self, VarError};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
  INIT.call_once(|| {
  tracing_subscriber::fmt()
    .with_ansi(false)
    .with_env_filter(
      tracing_subscriber::EnvFilter::from_default_env()
    ).init();
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
