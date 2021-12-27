mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    let srv = Server::new("localhost:3001");
    srv.run();
}
