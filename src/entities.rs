use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebToken {
    #[serde(default = "default_i32", rename = "DedeUserID")]
    pub dedeuserid: i32,
    #[serde(default = "default_string", rename = "DedeUserID__ckMd5")]
    pub dedeuserid_ckmd5: String,
    #[serde(default = "default_string", rename = "SESSDATA")]
    pub sessdata: String,
    #[serde(default = "default_string", rename = "bili_jct")]
    pub bili_jct: String,
    #[serde(default = "default_i32", rename = "Expires")]
    pub expires: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginQrData {
    #[serde(default = "default_string")]
    pub url: String,
    #[serde(default = "default_string", rename = "oauthKey")]
    pub oauth_key: String,
}

/// 因为API并不规范, 未登录成功返回数字, 登录成功返回字典, 所以进行了二次封装
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginQrInfo {
    // -1：密钥错误
    // -2：密钥超时
    // -4：未扫描
    // -5：未确认
    #[serde(default = "default_i32")]
    pub error_data: i32,
    #[serde(default = "default_string")]
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyInfo {
    pub mid: i64,
    pub name: String,
    pub sex: String,
    pub face: String,
    pub sign: String,
    pub rank: i64,
    pub level: i64,
    pub jointime: i64,
    pub moral: i64,
    pub silence: i64,
    pub email_status: i64,
    pub tel_status: i64,
    pub identification: i64,
    pub vip: Vip,
    pub pendant: Pendant,
    pub nameplate: Nameplate,
    pub official: Official,
    pub birthday: i64,
    pub is_tourist: i64,
    pub is_fake_account: i64,
    pub pin_prompting: i64,
    pub is_deleted: i64,
    pub in_reg_audit: i64,
    pub is_rip_user: bool,
    pub profession: Profession,
    pub face_nft: i64,
    pub face_nft_new: i64,
    pub is_senior_member: i64,
    pub level_exp: LevelExp,
    pub coins: i64,
    pub following: i64,
    pub follower: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vip {
    #[serde(rename = "type")]
    pub vip_type: i64,
    pub status: i64,
    pub due_date: i64,
    pub vip_pay_type: i64,
    pub theme_type: i64,
    pub label: VipLabel,
    pub avatar_subscript: i64,
    pub nickname_color: String,
    pub role: i64,
    pub avatar_subscript_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VipLabel {
    pub path: String,
    pub text: String,
    pub label_theme: String,
    pub text_color: String,
    pub bg_style: i64,
    pub bg_color: String,
    pub border_color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pendant {
    pub pid: i64,
    pub name: String,
    pub image: String,
    pub expire: i64,
    pub image_enhance: String,
    pub image_enhance_frame: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nameplate {
    pub nid: i64,
    pub name: String,
    pub image: String,
    pub image_small: String,
    pub level: String,
    pub condition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Official {
    pub role: i64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub official_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profession {
    pub id: i64,
    pub name: String,
    pub show_name: String,
    pub is_show: i64,
    pub category_one: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelExp {
    pub current_level: i64,
    pub current_min: i64,
    pub current_exp: i64,
    pub next_exp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoUrl {
    #[serde(default = "default_string")]
    pub from: String,
    #[serde(default = "default_string")]
    pub result: String,
    #[serde(default = "default_string")]
    pub message: String,
    #[serde(default = "default_i32")]
    pub quality: i32,
    #[serde(default = "default_string")]
    pub format: String,
    #[serde(default = "default_i32")]
    pub timelength: i32,
    #[serde(default = "default_string")]
    pub accept_format: String,
    #[serde(default = "default_vec")]
    pub accept_description: Vec<String>,
    #[serde(default = "default_vec")]
    pub accept_quality: Vec<i32>,
    #[serde(default = "default_i32")]
    pub video_codecid: i32,
    #[serde(default = "default_string")]
    pub seek_param: String,
    #[serde(default = "default_string")]
    pub seek_type: String,
    #[serde(default = "default_vec")]
    pub durl: Vec<Durl>,
    #[serde(default = "default_vec")]
    pub support_formats: Vec<SupportFormat>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Durl {
    #[serde(default = "default_i32")]
    pub order: i32,

    #[serde(default = "default_i64")]
    pub length: i64,

    #[serde(default = "default_i64")]
    pub size: i64,

    #[serde(default = "default_string")]
    pub ahead: String,

    #[serde(default = "default_string")]
    pub vhead: String,

    #[serde(default = "default_string")]
    pub url: String,

    #[serde(default = "default_vec")]
    pub backup_url: Vec<String>,
    // todo : highFormat
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportFormat {
    #[serde(default = "default_i32")]
    pub quality: i32,
    #[serde(default = "default_string")]
    pub format: String,
    #[serde(default = "default_string")]
    pub new_description: String,
    #[serde(default = "default_string")]
    pub display_desc: String,
    #[serde(default = "default_string")]
    pub superscript: String,
}

//////////////

// 240P 极速 仅mp4方式支持
pub const VIDEO_QUALITY_240P: VideoQuality = VideoQuality { code: 6 };

// 360P 流畅
pub const VIDEO_QUALITY_360P: VideoQuality = VideoQuality { code: 16 };

// 480P 清晰
pub const VIDEO_QUALITY_480P: VideoQuality = VideoQuality { code: 32 };

// 720P 高清 ;
// web端默认值 , B站前端需要登录才能选择，但是直接发送请求可以不登录就拿到720P的取流地址
// 无720P时则为720P60
pub const VIDEO_QUALITY_720P: VideoQuality = VideoQuality { code: 64 };

// 720P60 高帧率 ; 需要认证登录账号
pub const VIDEO_QUALITY_720P_60HZ: VideoQuality = VideoQuality { code: 74 };

// 1080P 高清
// TV端与APP端默认值, 需要认证登录账号
pub const VIDEO_QUALITY_1080P: VideoQuality = VideoQuality { code: 80 };

// 1080P+ 高码率	大多情况需求认证大会员账号
pub const VIDEO_QUALITY_1080P_HIGH: VideoQuality = VideoQuality { code: 112 };

// 1080P60 高帧率	大多情况需求认证大会员账号
pub const VIDEO_QUALITY_1080P_60HZ: VideoQuality = VideoQuality { code: 116 };

// 4K 超清	需要fnver&128=128且fourk=1  大多情况需求认证大会员账号
pub const VIDEO_QUALITY_4K: VideoQuality = VideoQuality { code: 120 };

// HDR 真彩色	仅支持dash方式
// 需要fnver&64=64
// 大多情况需求认证大会员账号
pub const VIDEO_QUALITY_HDR: VideoQuality = VideoQuality { code: 125 };

// 视频质量
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VideoQuality {
    pub code: i32,
}

impl Serialize for VideoQuality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.code)
    }
}

impl<'de> Deserialize<'de> for VideoQuality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(VideoQuality {
            code: Deserialize::deserialize(deserializer)?,
        })
    }
}

/////////

pub const FNVAL_FLV: i64 = 0;
pub const FNVAL_MP4: i64 = 1;
pub const FNVAL_DASH: i64 = 16;
pub const FNVAL_DASH_HDR: i64 = 64;
pub const FNVAL_4K: i64 = 128;
pub const FNVAL_DASH_DB: i64 = 256;
pub const FNVAL_DASH_VISION: i64 = 512;
pub const FNVAL_DASH_8K: i64 = 1024;
pub const FNVAL_DASH_AV1: i64 = 2048;

/////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvLoginQrData {
    #[serde(default = "default_string")]
    pub url: String,
    #[serde(default = "default_string")]
    pub auth_code: String,
}

/// 因为API并不规范, 未登录成功返回数字, 登录成功返回字典, 所以进行了二次封装
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginTvQrInfo {
    // 0：成功
    // -3：API校验密匙错误
    // -400：请求错误
    // 86038：二维码已失效
    // 86039：二维码尚未确认
    #[serde(default = "default_i32")]
    pub error_data: i32,
    #[serde(default = "default_i32")]
    pub mid: i32,
    #[serde(default = "default_string")]
    pub access_token: String,
    #[serde(default = "default_string")]
    pub refresh_token: String,
    #[serde(default = "default_i32")]
    pub expires_in: i32,
}

/////////

fn default_string() -> String {
    String::default()
}

fn default_i32() -> i32 {
    0
}

fn default_i64() -> i64 {
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
