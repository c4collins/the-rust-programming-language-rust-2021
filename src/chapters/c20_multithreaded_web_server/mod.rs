mod http;
mod routing;
mod server;
mod templates;
mod threading;

pub fn run() {
    let host = "127.0.0.1";
    let port = 7878;
    server::run(host, port);
}
