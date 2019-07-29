use actix::{Actor, Addr, Arbiter, Context, Handler, Message, Registry, ResponseFuture};

use crate::error::MyError;
//use crate::futures::Future;
//use crate::futures::IntoFuture;
use crate::server::User;
use crate::server::{GetUsers, Server};
use std::io;

use actix::prelude::*;
use futures::{future, Future, IntoFuture};

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
        println!("client started");

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
        //        Box::new(x.into_future())
        Box::new(Ok(vec![User::default()]).into_future())
    }
}
