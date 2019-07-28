use actix::{Actor, Addr, Arbiter, Context, Handler, Message, Registry, ResponseFuture};

use crate::error::MyError;
use crate::futures::Future;
use crate::server::User;
use crate::server::{GetUsers, Server};
use std::io;

pub struct Client {
    pub server: Addr<Server>,
}

pub struct ClientReq {}
impl Message for ClientReq {
    type Result = Result<Vec<User>, MyError>;
}

impl Actor for Client {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        //        self.server.send()

        //        let server: Server = Server::from_registry();
        //        let server: Server = Arbiter::registry().get::<Server>();
    }
}

impl Handler<ClientReq> for Client {
    type Result = ResponseFuture<Vec<User>, MyError>;

    fn handle(&mut self, msg: ClientReq, ctx: &mut Self::Context) -> Self::Result {
        let x = self.server.send(GetUsers {}).then(|rs| match rs {
            Ok(users) => {
                println!("users {:?}", users);
                Ok(users)
            }
            Err(e) => Err(MyError::Mess(format!("{:?}", e))),
        });
        Box::new(x)
    }
}
