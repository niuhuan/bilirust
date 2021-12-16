pub mod entities;
pub mod test;
pub mod types;

use entities::*;
use types::*;

const UA :&'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
const HOST_URL: &'static str = "https://api.bilibili.com";
/// 客户端
struct Client {
    agent: reqwest::Client,
}

/// 客户端
impl Client {
    /// 构造方法
    pub fn new() -> Self {
        Self {
            agent: reqwest::ClientBuilder::new().build().unwrap(),
        }
    }

    /// 请求并获得结果
    pub async fn request<T: for<'de> serde::Deserialize<'de>, R: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<R>,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let request = self
            .agent
            .request(method, format!("{}{}", HOST_URL, path).as_str())
            .header("User-Agent", UA);
        let request = match query {
            None => request,
            Some(query) => request.query(&query),
        };
        let resp = match body {
            None => request.send(),
            Some(body) => request.body(serde_json::to_string(&body)?).send(),
        };
        let resp = resp.await;
        match resp {
            Ok(resp) => {
                let response: Response<T> = serde_json::from_str(resp.text().await?.as_str())?;
                match &(response.code) {
                    0 => Ok(response.data.ok_or(Error::from("response empty"))?),
                    _ => Err(Box::new(Error::from(response.message))),
                }
            }
            Err(err) => Err(Box::new(Error::from(err.to_string()))),
        }
    }

    /// 获取BV信息
    pub async fn bv_info(&self, bvid: String) -> Result<BvInfo> {
        Ok(self
            .request(
                reqwest::Method::GET,
                "/x/web-interface/view",
                Option::Some(serde_json::json!({ "bvid": bvid })),
                None,
            )
            .await?)
    }
}
