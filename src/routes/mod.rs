use rocket::{Build, Rocket};

pub trait RocketRoutesAdd {
    fn routes_add(self, api_base: &str) -> Self;
}

impl RocketRoutesAdd for Rocket<Build> {
    fn routes_add(self, api_base: &str) -> Self {
        let path = format!("{}/cache", api_base);
        self.mount(path, routes![])
    }
}
