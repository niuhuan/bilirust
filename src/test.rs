use crate::{sign_form, Client, FNVAL_DASH, VIDEO_QUALITY_720P};

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
async fn test_login_qr() {
    let client = Client::new();
    match client.login_qr().await {
        Ok(info) => {
            println!("{}", serde_json::to_string(&info).unwrap())
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}

#[tokio::test]
async fn test_login_qr_info_parse() {
    let client = Client::new();
    match client.login_qr_info_parse_token("https://passport.biligame.com/crossDomain?DedeUserID=1&DedeUserID__ckMd5=2&Expires=3&SESSDATA=4&bili_jct=5&gourl=http%3A%2F%2Fwww.bilibili.com".to_string()) {
        Ok(info) => {
            println!("{}", serde_json::to_string(&info).unwrap())
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}

#[tokio::test]
async fn test_login_qr_info() {
    let client = Client::new();
    match client.login_qr_info("code".to_string()).await {
        Ok(info) => {
            println!("{}", serde_json::to_string(&info).unwrap())
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}

#[tokio::test]
async fn test_my_info() {
    let mut client = Client::new();
    client.login_set_sess_data("sess_data".to_string());
    match client.my_info().await {
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
        .bv_download_url(
            "BV1fK4y1t7hj".to_string(),
            196018899,
            FNVAL_DASH,
            VIDEO_QUALITY_720P,
        )
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

#[tokio::test]
async fn test_tv_login_qr() {
    let client = Client::new();
    match client.tv_login_qr().await {
        Ok(info) => {
            println!("{}", serde_json::to_string(&info).unwrap())
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}

#[tokio::test]
async fn test_tv_login_qr_info() {
    let client = Client::new();
    match client.tv_login_qr_info("code".to_string()).await {
        Ok(info) => {
            println!("{}", serde_json::to_string(&info).unwrap())
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}

#[tokio::test]
async fn test_videos_info() {
    let client = Client::new();
    match client.videos_info("ss5793".to_string()).await {
        Ok(info) => {
            println!("{}", serde_json::to_string(&info).unwrap())
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}
