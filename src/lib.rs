use std::str::FromStr;

use anyhow::anyhow;
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
            agent: reqwest::ClientBuilder::new()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36")
                .build().unwrap(),
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
                error_data: value.as_i64().ok_or(Error::msg("error format"))?,
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
                error_data: json["code"].as_i64().ok_or(Error::msg("error format"))?,
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
    pub async fn videos_info(&self, id: String) -> Result<web::SsState> {
        self.videos_info_by_url(format!("https://www.bilibili.com/bangumi/play/{}", id))
            .await
    }

    pub async fn videos_info_by_url(&self, url: String) -> Result<web::SsState> {
        let rsp = self
            .agent
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let start = r#"<script id="__NEXT_DATA__" type="application/json">"#;
        let stop = r#"</script>"#;
        let rsp: &str = match rsp.find(start) {
            None => return Err(Error::msg("not found (1)")),
            Some(index) => {
                let rsp = &rsp[(index + start.len())..];
                match rsp.find(stop) {
                    None => return Err(Error::msg("not found (2)")),
                    Some(index) => &rsp[..index],
                }
            }
        };
        let json: serde_json::Value = serde_json::from_str(rsp)?;
        let season_json = json
            .get("props")
            .ok_or(anyhow!("not found props"))?
            .get("pageProps")
            .ok_or(anyhow!("not found pageProps"))?
            .get("dehydratedState")
            .ok_or(anyhow!("not found dehydratedState"))?
            .get("queries")
            .ok_or(anyhow!("not found queries"))?
            .as_array()
            .ok_or(anyhow!("queries not array"))?
            .iter()
            .filter(|x| {
                if let Some(query_key) = x.get("queryKey") {
                    if let Some(query_key) = query_key.as_array() {
                        for x in query_key {
                            if let Some(x) = x.as_str() {
                                if x == "pgc/view/web/season" {
                                    return true;
                                }
                            }
                        }
                    }
                }
                return false;
            })
            .nth(0)
            .ok_or(anyhow!("not found pgc/view/web/season"))?;
        let season_json = season_json
            .get("state")
            .ok_or(anyhow!("query not state"))?
            .get("data")
            .ok_or(anyhow!("state not data"))?;
        Ok(from_value(season_json.clone())?)
    }

    pub async fn user_info(&self, mid: i64) -> Result<UserInfo> {
        Ok(self
            .request_api(
                reqwest::Method::GET,
                "/x/space/acc/info",
                Some(serde_json::json!({
                    "mid": mid,
                    "token":"",
                    "platform":"web",
                    "jsonp":"jsonp",
                })),
                None,
            )
            .await?)
    }

    // https://space.bilibili.com/{mid}/channel/series
    // page_num 1 开始
    // page_size 请使用30
    pub async fn seasons_series_list_data(
        &self,
        mid: i64,
        page_num: i64,
        page_size: i64,
    ) -> Result<SeasonsSeriesListData> {
        Ok(self
            .request_api(
                reqwest::Method::GET,
                "/x/polymer/space/seasons_series_list",
                Some(serde_json::json!({
                    "mid": mid,
                    "page_num":page_num,
                    "page_size":page_size,
                })),
                None,
            )
            .await?)
    }

    // https://space.bilibili.com/{mid}/channel/series
    // page_num 1 开始
    // page_size 请使用30
    pub async fn seasons_series_list(
        &self,
        mid: i64,
        page_num: i64,
        page_size: i64,
    ) -> Result<SeasonsSeriesList> {
        Ok(self
            .seasons_series_list_data(mid, page_num, page_size)
            .await?
            .items_lists)
    }

    // https://space.bilibili.com/{mid}/channel/collectiondetail?sid={sid}
    // page_num 1 开始
    // page_size 请使用30
    pub async fn collection_video_page(
        &self,
        mid: i64,
        sid: i64,
        sort_reverse: bool,
        page_num: i64,
        page_size: i64,
    ) -> Result<CollectionDetailPage> {
        Ok(self
            .request_api(
                reqwest::Method::GET,
                "/x/polymer/space/seasons_archives_list",
                Some(serde_json::json!({
                    "mid": mid,
                    "season_id":sid,
                    "sort_reverse":sort_reverse,
                    "page_num":page_num,
                    "page_size":page_size,
                })),
                None,
            )
            .await?)
    }

    pub async fn series_info(&self, series_id: i64) -> Result<SeriesVideoInfoData> {
        Ok(self
            .request_api(
                reqwest::Method::GET,
                "/x/series/series",
                Some(serde_json::json!({
                    "series_id": series_id,
                })),
                None,
            )
            .await?)
    }

    // https://space.bilibili.com/{mid}/favlist?fid={fid}
    // page_num 1 开始
    // page_size 请使用20
    pub async fn fav_list_page(
        &self,
        fid: i64,
        page_num: i64,
        page_size: i64,
        keyword: Option<String>,
        order: FavListOrder,
    ) -> Result<FavListPage> {
        Ok(self
            .request_api(
                reqwest::Method::GET,
                "/x/v3/fav/resource/list",
                Some(serde_json::json!({
                    "media_id": fid,
                    "pn":page_num,
                    "ps":page_size,
                    "keyword": if let Some(keyword) = keyword {
                        keyword
                    } else {
                        "".to_string()
                    },
                    "order": order, // mtime (最新收藏), view (最多播放)， pubtime (发布时间)，
                    "type":0,
                    "platform":"web",
                })),
                None,
            )
            .await?)
    }
}

#[cfg(test)]
pub mod test;
