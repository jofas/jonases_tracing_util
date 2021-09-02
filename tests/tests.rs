use jonases_tracing_util::{log_simple_err_callback, scoped_logger};

use actix_web::App;

#[actix_rt::test]
async fn test_building_scoped_logger() {
  App::new().wrap_fn(scoped_logger!());
  assert!(true);
}

#[test]
fn log_message_with_static_str() {
  assert!(Result::<(), ()>::Err(())
    .map_err(log_simple_err_callback("simple static log message",))
    .is_err());
}

#[test]
fn log_message_with_formatted_string() {
  assert!(Result::<(), ()>::Err(())
    .map_err(log_simple_err_callback(format!(
      "logging with formatting: {}",
      1
    ),))
    .is_err());
}
