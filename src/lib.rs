use std::str::FromStr;

pub use anyhow::{Error, Result};
use chrono::Timelike;
pub use entities::*;
pub use utils::*;

pub mod entities;
pub mod utils;

const API_HOST_URL: &'static str = "https://api.bilibili.com";
const PASSPORT_HOST_URL: &'static str = "https://passport.bilibili.com";

const APP_KEY_TV: &'static str = "4409e2ce8ffd12b8";
const APP_SEC_TV: &'static str = "59b43e04ad6965f34319062b478f83dd";
const LOCAL_ID_TV: &'static str = "0";

/// 客户端
#[derive(Clone)]
pub struct Client {
    agent: reqwest::Client,
    sess_data: Option<String>,
}

/// 客户端
impl Client {
    /// 构造方法
    pub fn new() -> Self {
        Self {
            agent: reqwest::ClientBuilder::new().build().unwrap(),
            sess_data: None,
        }
    }

    /// 请求并获得结果
    pub async fn request_api<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<serde_json::Value>,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let request = self
            .agent
            .request(method, format!("{}{}", API_HOST_URL, path).as_str());
        let request = match &self.sess_data {
            Some(web_token) => request.header("Cookie", format!("SESSDATA={}", web_token,)),
            None => request,
        };
        let request = match query {
            None => request,
            Some(query) => request.query(&query),
        };
        let resp = match body {
            None => request.send(),
            Some(body) => request.body(serde_json::to_string(&body)?).send(),
        };
        let resp = resp.await?;
        let body = resp.text().await?;
        let response: Response<T> = from_str(&body)?;
        match &(response.code) {
            0 => Ok(response.data.ok_or(Error::msg("response empty"))?),
            _ => Err(anyhow::Error::msg(response.message)),
        }
    }

    /// 请求PASSPORT服务器并获得结果
    /// 因为接口返回格式不统一, 所以返回json, 方法自行处理结果
    pub async fn request_passport(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<serde_json::Value>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let request = self
            .agent
            .request(method, format!("{}{}", PASSPORT_HOST_URL, path).as_str());
        let request = match query {
            None => request,
            Some(query) => request.query(&query),
        };
        let resp = match body {
            None => request.send(),
            Some(body) => request.form(&body).send(),
        };
        let resp = resp.await?;
        let body = resp.text().await?;
        let json: serde_json::Value = from_str(body.as_str())?;
        Ok(json)
    }

    /// WEB二维码登录 - 申请二维码
    /// 此返回结构略有不同, 所以进行了自定义封装
    /// code为0成功
    pub async fn login_qr(&self) -> Result<LoginQrData> {
        let json = self
            .request_passport(reqwest::Method::GET, "/qrcode/getLoginUrl", None, None)
            .await?;
        let code = &json["code"];
        if !code.is_i64() {
            return Err(Error::msg("err code format"));
        }
        if code.as_i64().unwrap() != 0 {
            return Err(Error::msg("error"));
        }
        let data = json["data"].clone();
        Ok(serde_json::from_value(data)?)
    }

    /// WEB二维码登录 - 获取Web二维码登录信息
    pub async fn login_qr_info(&self, oauth_key: String) -> Result<LoginQrInfo> {
        let json = self
            .request_passport(
                reqwest::Method::POST,
                "/qrcode/getLoginInfo",
                None,
                Option::Some(serde_json::json!({ "oauthKey": oauth_key })),
            )
            .await?;
        let value = json["data"].clone();
        if value.is_i64() {
            Ok(LoginQrInfo {
                error_data: value.as_i64().ok_or(Error::msg("error format"))? as i32,
                url: String::default(),
            })
        } else {
            let info = LoginQrInfo {
                error_data: 0,
                url: value["url"]
                    .as_str()
                    .ok_or(Error::msg("error format"))?
                    .to_string(),
            };
            Ok(info)
        }
    }

    /// 将url转换为token
    pub fn login_qr_info_parse_token(&self, url: String) -> Result<WebToken> {
        let regex = regex::Regex::new("^.+crossDomain\\?DedeUserID=(\\d+)&DedeUserID__ckMd5=([a-z0-9]+)&Expires=(\\d+)&SESSDATA=([^&]+)&bili_jct=([^&]+)&.+$")?;
        let match_url = regex
            .captures(url.as_str())
            .ok_or(Error::msg("not match"))?;
        let uid = match_url.get(1).ok_or(Error::msg("not match 1"))?.as_str();
        let md5 = match_url.get(2).ok_or(Error::msg("not match 2"))?.as_str();
        let exp = match_url.get(3).ok_or(Error::msg("not match 3"))?.as_str();
        let sess = match_url.get(4).ok_or(Error::msg("not match 4"))?.as_str();
        let jct = match_url.get(5).ok_or(Error::msg("not match 5"))?.as_str();
        Ok(WebToken {
            dedeuserid: FromStr::from_str(uid)?,
            dedeuserid_ckmd5: md5.to_string(),
            sessdata: sess.to_string(),
            bili_jct: jct.to_string(),
            expires: FromStr::from_str(exp)?,
        })
    }

    /// 登录 (注入token)
    pub fn login_set_sess_data(&mut self, sess_data: String) {
        self.sess_data = Some(sess_data);
    }

    /// 个人信息, 登录后才能使用
    pub async fn my_info(&self) -> Result<MyInfo> {
        Ok(self
            .request_api(reqwest::Method::GET, "/x/space/myinfo", None, None)
            .await?)
    }

    /// 获取BV信息
    pub async fn bv_info(&self, bvid: String) -> Result<BvInfo> {
        Ok(self
            .request_api(
                reqwest::Method::GET,
                "/x/web-interface/view",
                Option::Some(serde_json::json!({ "bvid": bvid })),
                None,
            )
            .await?)
    }

    /// 获取BV信息(用AV号)
    pub async fn av_info(&self, avid: i64) -> Result<BvInfo> {
        self.bv_info(av_to_bv(avid)).await
    }

    /// 获取下载地址
    pub async fn bv_download_url(
        &self,
        bvid: String,
        cid: i64,
        fnval: i64,
        video_quality: VideoQuality,
    ) -> Result<VideoUrl> {
        Ok(self
            .request_api(
                reqwest::Method::GET,
                "/x/player/playurl",
                Option::Some(serde_json::json!({
                    "bvid": bvid,
                    "cid":cid,
                    "qn":video_quality.code,
                    "fnval":fnval,
                    "fnver":0,
                    "fourk":1,
                })),
                None,
            )
            .await?)
    }

    /// TV登录申请二维码
    pub async fn tv_login_qr(&self) -> Result<TvLoginQrData> {
        let json = self
            .request_passport(
                reqwest::Method::POST,
                "/x/passport-tv-login/qrcode/auth_code",
                None,
                Option::Some(sign_form(
                    serde_json::json!({
                        "appkey": APP_KEY_TV,
                        "local_id": LOCAL_ID_TV,
                        "ts": format!("{}", chrono::Local::now().second()),
                    }),
                    APP_SEC_TV,
                )?),
            )
            .await?;
        let code = &json["code"];
        if !code.is_i64() {
            return Err(Error::msg("err code format"));
        }
        if code.as_i64().unwrap() != 0 {
            // todo
            return Err(Error::msg("error"));
        }
        let data = json["data"].clone();
        Ok(serde_json::from_value(data)?)
    }

    /// 获取TV二维码登录信息
    pub async fn tv_login_qr_info(&self, auth_code: String) -> Result<LoginTvQrInfo> {
        let json = self
            .request_passport(
                reqwest::Method::POST,
                "/x/passport-tv-login/qrcode/poll",
                None,
                Option::Some(sign_form(
                    serde_json::json!({
                        "appkey": APP_KEY_TV,
                        "auth_code": auth_code,
                        "local_id": LOCAL_ID_TV,
                        "ts": format!("{}", chrono::Local::now().second()),
                    }),
                    APP_SEC_TV,
                )?),
            )
            .await?;
        let value = json["data"].clone();
        if value.is_null() {
            Ok(LoginTvQrInfo {
                error_data: json["code"].as_i64().ok_or(Error::msg("error format"))? as i32,
                mid: 0,
                access_token: "".to_string(),
                refresh_token: "".to_string(),
                expires_in: 0,
            })
        } else {
            Ok(serde_json::from_value(value)?)
        }
    }

    /// 获取视频信息
    /// id: 例如 ep1234 ss1234
    pub async fn videos_info(&self, id: String) -> Result<SsState> {
        let rsp = self
            .agent
            .get(format!("https://www.bilibili.com/bangumi/play/{}", id))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let rsp: &str = match rsp.find("window.__INITIAL_STATE__=") {
            None => return Err(Error::msg("not found (1)")),
            Some(index) => {
                let rsp = &rsp[(index + "window.__INITIAL_STATE__=".len())..];
                match rsp.find(";(function()") {
                    None => return Err(Error::msg("not found (2)")),
                    Some(index) => &rsp[..index],
                }
            }
        };
        Ok(from_str(rsp)?)
    }
}

#[cfg(test)]
pub mod test;
