BILIRUST
===========
RUST哔哩哔哩客户端

## 实现功能

- [x] 用户
    - [x] 使用WEB方式扫码获取SessionData
    - [x] 获取个人信息
- [x] 视频
    - [x] 通过BVID/AVID获取视频信息
    - [x] 获取视频的下载地址
- [x] 客户端接口支持
    - [x] 使用TV方式扫码获取SessionData/refresh_token

## 使用方法

```rust
async fn main() {
    // 创建一个客户端
    let mut client = Client::new();
    // 扫码登录等
    client.login_qr().await;
    client.login_qr_info("code".to_string()).await;
    client.login_qr_info_parse_token("url".to_string());
    client.login_set_sess_data("sess_data");  // mut
    // 获取自己的个人信息
    client.my_info().await;
    // 获取视频的信息
    client.bv_info("BV1TS4y1Q7Y9".to_string()).await;
    // 获取视频的下载地址
    client.bv_download_url("BV1TS4y1Q7Y9".to_string(), 459566105, VIDEO_QUALITY_720P).await;
    // TV扫码登录
    client.tv_login_qr().await;
    client.tv_login_qr_info("code".to_string()).await;
}
```
