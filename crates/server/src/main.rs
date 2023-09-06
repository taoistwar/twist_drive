use twist_drive_server::startup;

#[tokio::main]
async fn main() {
    startup().await;
}
