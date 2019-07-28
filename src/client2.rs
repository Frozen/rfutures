//mod server;
use crate::client::{Client, ClientReq};
use crate::futures::Future;
use actix::{Actor, Addr, Context};

pub struct Client2 {
    pub server: Addr<Client>,
}
impl Actor for Client2 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server.send(ClientReq {}).map(|users| {
            println!("userss {:?}", users);
            //            users
        })

        //        self.server.send()

        //        let server: Server = Server::from_registry();
        //        let server: Server = Arbiter::registry().get::<Server>();
    }
}
