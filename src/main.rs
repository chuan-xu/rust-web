use axum::{
    AddExtensionLayer,
    handler::get,
    handler::post,
    Router
};

use num_cpus;
use serde::Deserialize;
use std::fs::File;
use std::{io::Read, net::SocketAddr, sync::Arc};
use tokio::runtime::Builder;
use tower::layer::layer_fn;

fn main() {
    let cfg = init();
    let basic_rt = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(cfg.cpu_nums)
        .build()
        .unwrap();
    basic_rt.block_on(app(&cfg));
}

#[derive(Deserialize)]
struct CfgTomal {
    cpu_nums: usize,
    listen_addr: SocketAddr,
    redis_addr: SocketAddr,
    posg_addr: String,
}

#[derive(Debug)]
struct BaseCfg {
    cpu_nums: usize,
    listen_addr: SocketAddr,
    redis_addr: String,
    posg_addr: String,
}

impl BaseCfg {
    fn new(cfg: CfgTomal) ->Self {
        BaseCfg{
            cpu_nums: cfg.cpu_nums,
            listen_addr: cfg.listen_addr,
            redis_addr: "redis://".to_string() + &cfg.redis_addr.to_string(),
            posg_addr: cfg.posg_addr
        }
    }
    fn set_cpu_nums(&mut self, nums: usize) {
        self.cpu_nums = nums;
    }
}

fn init() -> BaseCfg {
    if std::env::var_os("RUST_LOG ").is_none() {
        std::env::set_var("RUST_LOG", "debug ")
    }
    tracing_subscriber::fmt::init();
    let mut f = File::open("src/conf.toml").unwrap();
    let mut toml_str = String::new();
    f.read_to_string(&mut toml_str).unwrap();
    let cfg: CfgTomal = toml::from_str(&toml_str).unwrap();
    let mut base = BaseCfg::new(cfg);
    if base.cpu_nums == 0 {
        base.set_cpu_nums(num_cpus::get());
    }
    println!("{:?}", base);
    base
}

mod cmw;

use cmw::mym::MyMiddleware;

mod dbs;
mod mode;

mod req;

use req::{auth, index};

mod tests;

async fn app(cfg: &BaseCfg) {
    let pool = dbs::DBS::new(&cfg.posg_addr, &cfg.redis_addr).await;
    let pool_arc = Arc::new(pool);
    let mid = layer_fn(|inner| MyMiddleware { inner });
    let app = Router::new()
        .route("/", get(index::home))
        .route("/test", post(auth::test_handler))
        .route("/json", post(auth::test_json))
        .layer(AddExtensionLayer::new(pool_arc))
        .layer(mid);
    axum::Server::bind(&cfg.listen_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

