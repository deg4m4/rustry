mod server;
pub mod service;

fn main() {

    tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(
        async{
            server::create_server().await;
        }
    );

}
