use actix::{Actor, ArbiterService, Context, Handler, Message};
use std::io;

#[derive(Default, Debug)]
pub struct User {}

pub struct GetUsers {}
impl Message for GetUsers {
    //    type Result = ResponseFuture<Vec<User>, io::Error>;
    type Result = Result<Vec<User>, io::Error>;
    //    type Result = Vec<User>;
}

pub struct Server {}
impl Handler<GetUsers> for Server {
    type Result = Result<Vec<User>, io::Error>;

    fn handle(&mut self, msg: GetUsers, ctx: &mut Self::Context) -> Self::Result {
        //        Box::new(futures::future::ok(vec![User {}]))
        Ok(vec![User::default()])
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

//impl ArbiterService for Server {
//    fn service_started(&mut self, ctx: &mut Context<Self>) {
//        println!("Server Service started");
//    }
//}
