use reqwest::Client;
use tokio::sync::OnceCell;
use crate::config::get_config_clone;

static CLIENT_CELL: OnceCell<Client> = OnceCell::const_new();

pub async fn get_client() -> &'static Client {
    CLIENT_CELL.get_or_init(|| async {
        let config = get_config_clone().await;
        if (config.proxy.enabled) {
            let scheme = format!("socks5://{}:{}", config.proxy.address, config.proxy.port);
            let proxy = reqwest::Proxy::all(scheme).unwrap();
            Client::builder()
                .proxy(proxy)
                .build()
                .unwrap()
        } else {
            Client::new()
        }

    }).await
}