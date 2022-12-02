use pm3::rpc::Pm3;
use tarpc::trace::TraceId;
use std::net::SocketAddr;
use std::sync::Arc;
use tarpc::context::Context;
use tokio::sync::Mutex;

use crate::log_buffer::ClientId;
use crate::pool::Pool;

#[derive(Clone)]
pub struct Pm3Server {
    addr: SocketAddr,
    pool: Arc<Mutex<Pool>>,
}

impl Pm3Server {
    pub fn new(addr: SocketAddr, pool: Arc<Mutex<Pool>>) -> Self {
        Pm3Server { addr, pool }
    }
}

#[tarpc::server]
impl Pm3 for Pm3Server {
    async fn start(self, _: Context, command: String, args: Vec<String>) {
        let mut pool = self.pool.lock().await;
        let args: Vec<_> = args.iter().map(String::as_str).collect();
        pool.spawn(&command, &args);
        format!("executed command");
    }

    async fn get_log(self, context: Context) -> Vec<String> {
        let mut pool = self.pool.lock().await;
        let trace = context.trace_id().clone().into();

        if let Some(log) = pool.log(0, ClientId(trace)) {
            log
        } else {
            vec![]
        }
    }

    async fn hello(self, _: Context, name: String) -> String {
        format!("Hello, {name}! You are connected from {}", self.addr)
    }

    async fn ping(self, _: Context) {
        ()
    }

    async fn kill(self, _: Context) {
        println!("daemon: kill");
        std::process::exit(0);
    }
}
