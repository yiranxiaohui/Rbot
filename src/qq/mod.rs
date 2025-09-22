mod token;

pub async fn init() {
    token::init().await;
}