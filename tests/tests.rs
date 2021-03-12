use jonases_tracing_util::scoped_logger;

use actix_web::App;

#[actix_rt::test]
async fn test_building_scoped_logger() {
  App::new().wrap_fn(scoped_logger!());
  assert!(true);
}
