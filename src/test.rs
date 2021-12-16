#[cfg(test)]
mod tests {
    use crate::{sign_form, Client, VIDEO_QUALITY_720P};

    #[tokio::test]
    async fn test_sign_form() {
        match sign_form(serde_json::json!({"a":"a","b":true}), "123") {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    #[tokio::test]
    async fn test_login_web_qr() {
        let client = Client::new();
        match client.login_web_qr().await {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    #[tokio::test]
    async fn test_login_web_qr_info() {
        let client = Client::new();
        match client.login_web_qr_info("code".to_string()).await {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    #[tokio::test]
    async fn test_login_tv_qr() {
        let client = Client::new();
        match client.login_tv_qr().await {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    #[tokio::test]
    async fn test_login_tv_qr_info() {
        let client = Client::new();
        match client.login_tv_qr_info("code".to_string()).await {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    #[tokio::test]
    async fn test_bv_info_info() {
        let client = Client::new();
        match client.bv_info("BV1TS4y1Q7Y9".to_string()).await {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    #[tokio::test]
    async fn test_av_info() {
        let client = Client::new();
        match client.av_info(722327931).await {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }

    #[tokio::test]
    async fn test_bv_download_url() {
        let client = Client::new();
        match client
            .bv_download_url("BV1TS4y1Q7Y9".to_string(), 459566105, VIDEO_QUALITY_720P)
            .await
        {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap())
            }
            Err(err) => {
                println!("{}", err)
            }
        };
    }
}
