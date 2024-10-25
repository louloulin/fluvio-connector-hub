use fluvio_connector_common::{connector, secret::SecretString};

#[connector(config, name = "nats")]
#[derive(Debug)]
pub(crate) struct NatsConfig {
    pub url: SecretString,  // NATS 服务器 URL

    pub subject: String,    // NATS 主题

    pub queue_group: Option<String>,  // 可选的队列组
}
