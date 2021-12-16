#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_bv_info() {
        let client = Client::new();
        match client.bv_info("1TS4y1Q7Y9".to_string()).await {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }
}
