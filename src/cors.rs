use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Build, Request, Response, Rocket};

pub trait RocketCorsEnabler {
    fn enable_cors(self) -> Self;
}

impl RocketCorsEnabler for Rocket<Build> {
    fn enable_cors(self) -> Self {
        self.attach(CORS)
    }
}

pub struct CORS;

#[async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS headers injector",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // CORS headers
        // TODO! Change Access-Control-Allow-Origin
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        // Preflight request handling
        if request.method() == Method::Options {
            response.set_status(Status::NoContent)
        }
    }
}
