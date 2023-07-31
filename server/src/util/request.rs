/// request add some feature on request
use chrono::{DateTime, Utc};
use log::info;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
pub struct Context {}

pub struct Start(DateTime<Utc>);
impl std::fmt::Display for Start {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rocket::async_trait]
impl Fairing for Context {
    fn info(&self) -> Info {
        Info {
            name: "Request Context",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| Start(Utc::now()));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let start = req.local_cache(|| Start(Utc::now()));
        let cost = Utc::now().timestamp_micros() - start.0.timestamp_micros();
        info!(target:"server", "{} cost[{}] method[{}] uri[{}] client[{:?}] status[{}]", start, cost, req.method(), req.uri(), req.client_ip(), res.status());
    }
}
