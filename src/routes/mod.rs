use rocket::{Build, Rocket};

mod create;
use create::create_cache;

mod view;
use view::view_cache;
use view::view_caches;

mod delete;
use delete::delete_cache;
pub trait RocketRoutesAdd {
    fn routes_add(self, api_base: &str) -> Self;
}

impl RocketRoutesAdd for Rocket<Build> {
    fn routes_add(self, api_base: &str) -> Self {
        let path = format!("{}/cache", api_base);
        self.mount(
            path,
            routes![create_cache, view_caches, view_cache, delete_cache],
        )
    }
}
