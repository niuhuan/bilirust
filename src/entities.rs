use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    #[serde(default = "default_i32")]
    pub code: i32,
    #[serde(default = "default_string")]
    pub message: String,
    #[serde(default = "default_i32")]
    pub ttl: i32,
    #[serde(default = "default_option")]
    pub data: Option<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BvInfo {
    #[serde(default = "default_string")]
    pub bvid: String,
    #[serde(default = "default_i32")]
    pub aid: i32,
    #[serde(default = "default_i32")]
    pub videos: i32,
    #[serde(default = "default_i32")]
    pub tid: i32,
    #[serde(default = "default_i32")]
    pub copyright: i32,
    #[serde(default = "default_string")]
    pub pic: String,
    #[serde(default = "default_string")]
    pub title: String,
    #[serde(default = "default_i32")]
    pub ctime: i32,
    #[serde(default = "default_string")]
    pub desc: String,
    #[serde(default = "default_vec")]
    pub desc_v2: Vec<DescV2>,
    #[serde(default = "default_i32")]
    pub state: i32,
    #[serde(default = "default_i32")]
    pub duration: i32,
    #[serde(default = "default_string")]
    pub dynamic: String,
    #[serde(default = "default_i32")]
    pub cid: i32,
    #[serde(default = "default_dimension")]
    pub dimension: Dimension,
    #[serde(default = "default_bool")]
    pub no_cache: bool,
    #[serde(default = "default_vec")]
    pub pages: Vec<Page>,
    // todo subtitle...
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DescV2 {
    #[serde(default = "default_string")]
    pub raw_text: String,
    #[serde(default = "default_i32", rename = "type")]
    pub desc_type: i32,
    #[serde(default = "default_i32")]
    pub biz_id: i32,
    #[serde(default = "default_rights")]
    pub rights: Rights,
    #[serde(default = "default_owner")]
    pub owner: Owner,
    #[serde(default = "default_stat")]
    pub stat: Stat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rights {
    #[serde(default = "default_i32")]
    bp: i32,
    #[serde(default = "default_i32")]
    elec: i32,
    #[serde(default = "default_i32")]
    download: i32,
    #[serde(default = "default_i32")]
    movie: i32,
    #[serde(default = "default_i32")]
    pay: i32,
    #[serde(default = "default_i32")]
    hd5: i32,
    #[serde(default = "default_i32")]
    no_reprint: i32,
    #[serde(default = "default_i32")]
    autoplay: i32,
    #[serde(default = "default_i32")]
    ugc_pay: i32,
    #[serde(default = "default_i32")]
    is_cooperation: i32,
    #[serde(default = "default_i32")]
    ugc_pay_preview: i32,
    #[serde(default = "default_i32")]
    no_background: i32,
    #[serde(default = "default_i32")]
    clean_mode: i32,
    #[serde(default = "default_i32")]
    is_stein_gate: i32,
    #[serde(default = "default_i32")]
    is_360: i32,
    #[serde(default = "default_i32")]
    no_share: i32,
}

fn default_rights() -> Rights {
    Rights {
        bp: 0,
        elec: 0,
        download: 0,
        movie: 0,
        pay: 0,
        hd5: 0,
        no_reprint: 0,
        autoplay: 0,
        ugc_pay: 0,
        is_cooperation: 0,
        ugc_pay_preview: 0,
        no_background: 0,
        clean_mode: 0,
        is_stein_gate: 0,
        is_360: 0,
        no_share: 0,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    #[serde(default = "default_i32", rename = "type")]
    pub mid: i32,
    #[serde(default = "default_string")]
    pub name: String,
    #[serde(default = "default_string")]
    pub face: String,
}

fn default_owner() -> Owner {
    Owner {
        mid: 0,
        name: String::default(),
        face: String::default(),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stat {
    #[serde(default = "default_i32")]
    pub aid: i32,
    #[serde(default = "default_i32")]
    pub view: i32,
    #[serde(default = "default_i32")]
    pub danmaku: i32,
    #[serde(default = "default_i32")]
    pub reply: i32,
    #[serde(default = "default_i32")]
    pub favorite: i32,
    #[serde(default = "default_i32")]
    pub coin: i32,
    #[serde(default = "default_i32")]
    pub share: i32,
    #[serde(default = "default_i32")]
    pub now_rank: i32,
    #[serde(default = "default_i32")]
    pub his_rank: i32,
    #[serde(default = "default_i32")]
    pub like: i32,
    #[serde(default = "default_i32")]
    pub dislike: i32,
    #[serde(default = "default_string")]
    pub evaluation: String,
    #[serde(default = "default_string")]
    pub argue_msg: String,
}

fn default_stat() -> Stat {
    Stat {
        aid: 0,
        view: 0,
        danmaku: 0,
        reply: 0,
        favorite: 0,
        coin: 0,
        share: 0,
        now_rank: 0,
        his_rank: 0,
        like: 0,
        dislike: 0,
        evaluation: String::default(),
        argue_msg: String::default(),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dimension {
    #[serde(default = "default_i32")]
    pub width: i32,
    #[serde(default = "default_i32")]
    pub height: i32,
    #[serde(default = "default_i32")]
    pub rotate: i32,
}

fn default_dimension() -> Dimension {
    Dimension {
        width: 0,
        height: 0,
        rotate: 0,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(default = "default_i32")]
    pub cid: i32,
    #[serde(default = "default_i32")]
    pub page: i32,
    #[serde(default = "default_string")]
    pub from: String,
    #[serde(default = "default_string")]
    pub part: String,
    #[serde(default = "default_i32")]
    pub duration: i32,
    #[serde(default = "default_string")]
    pub vid: String,
    #[serde(default = "default_string")]
    pub weblink: String,
    #[serde(default = "default_dimension")]
    pub dimension: Dimension,
    #[serde(default = "default_string")]
    pub first_frame: String,
}

fn default_string() -> String {
    String::default()
}

fn default_i32() -> i32 {
    0
}

fn default_bool() -> bool {
    false
}

fn default_option<T>() -> Option<T> {
    Option::None
}

fn default_vec<T>() -> Vec<T> {
    vec![]
}
