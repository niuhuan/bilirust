BILIRUST
===========
RUST哔哩哔哩客户端

## 实现功能

- [x] 用户
    - [x] 使用WEB方式扫码获取SessionData
    - [x] 使用TV方式扫码获取SessionData/refresh_token
- [x] 视频
    - [x] 通过BVID/AVID获取视频信息
    - [x] 获取视频的下载地址

## 使用方法

```rust
async fn main() {
    // 创建一个客户端
    let client = Client::new();
    // 扫码登录等
    client.login_web_qr().await;
    client.login_web_qr_info("code".to_string()).await;
    client.login_tv_qr().await;
    client.login_tv_qr_info("code".to_string()).await;
    // 获取视频的信息
    client.bv_info("BV1TS4y1Q7Y9".to_string()).await;
    // 获取视频的下载地址
    client.bv_download_url("BV1TS4y1Q7Y9".to_string(), 459566105, VIDEO_QUALITY_720P).await;
}
```
