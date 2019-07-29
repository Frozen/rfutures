extern crate actix;
extern crate futures;
extern crate tokio;

mod client;
mod client2;
mod error;
mod server;

use actix::{Actor, Context, Handler, Message, ResponseFuture, System};
use futures::{Async, Future, IntoFuture};
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::io;
use tokio::reactor::Handle;

use server::Server;

fn main() {
    let sys = System::new("test");

    let server = Server::create(|ctx| Server {});

    let client = client::Client { server };

    let addr2 = client.start();

    let client2 = client2::Client2 { server: addr2 };

    client2.start();
    //
    let r = sys.run();
}
