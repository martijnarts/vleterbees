use app::*;
use server::run;

#[tokio::main]
pub async fn main() {
    run(App).await.unwrap()
}
