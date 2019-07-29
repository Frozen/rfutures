//mod server;
use crate::client::{Client, ClientReq};
use crate::futures::Future;
use actix::{spawn, Actor, Addr, AsyncContext, Context, WrapFuture};
//use actix_service::ServiceExt;

pub struct Client2 {
    pub server: Addr<Client>,
}
impl Actor for Client2 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let t = self
            .server
            .send(ClientReq {})
            .map(|users| {
                println!("userss {:?}", users);
                //            users
            })
            .map_err(|e| {
                println!("{}", e);
            })
            .into_actor(self);
        //            .into_future();
        ctx.spawn(t);
        //        spawn(t);

        //        self.server.send()

        //        let server: Server = Server::from_registry();
        //        let server: Server = Arbiter::registry().get::<Server>();
    }
}
