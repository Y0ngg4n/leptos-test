pub mod app;
#[cfg(feature = "ssr")]
pub mod models;
#[cfg(feature = "ssr")]
pub mod schema;

#[cfg(feature = "ssr")]
use diesel::sqlite::*;
#[cfg(feature = "ssr")]
use diesel_async::pooled_connection::bb8::*;
#[cfg(feature = "ssr")]
use diesel_async::sync_connection_wrapper::*;
use dotenvy::dotenv;
use std::env;

#[cfg(feature = "ssr")]
pub async fn establish_connection() -> Pool<SyncConnectionWrapper<SqliteConnection>> {
    use diesel::prelude::*;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::sync_connection_wrapper::*;
    use diesel_async::AsyncConnection;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config =
        AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(database_url);
    return Pool::builder()
        .build(config)
        .await
        .unwrap_or_else(|_| panic!("Error connecting the database!"));
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
