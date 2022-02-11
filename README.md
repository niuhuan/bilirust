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
  
    // 登录相关
    // 扫码登录
    let qr_data = client.login_qr().await.unwrap();
    // 扫码后访问, (如未扫码或其他错误, 则error_data非0)
    let info = client.login_qr_info(qr_data.oauth_key.clone()).await.unwrap();
    // 解析扫码结果
    let web_token = client.login_qr_info_parse_token(info.url.to_string()).unwrap();
    // 注入token
    client.login_set_sess_data(web_token.sess_data);  // mut
  
    // 接口相关
    // 获取自己的个人信息 (需要登录)
    client.my_info().await;
    // 使用EP/SS获取信息 
    client.videos_info("ep1234".to_string()).await;
    client.videos_info("ss1234".to_string()).await; 
    // 获取视频的信息
    client.bv_info("BV1TS4y1Q7Y9".to_string()).await;
    // 获取视频的下载地址 (1080P以上需要登录) (FNVAL可以使用位或一起使用) (FLV可能会被分段)
    client.bv_download_url("BV1TS4y1Q7Y9".to_string(), 459566105, FNVAL_DASH, VIDEO_QUALITY_720P).await;
    
    // 其他, 仅实现, 暂时没什么用
    // TV扫码登录
    client.tv_login_qr().await;
    client.tv_login_qr_info("code".to_string()).await;
}
```
