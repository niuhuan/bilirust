use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebToken {
    #[serde(default = "default_i64", rename = "DedeUserID")]
    pub dedeuserid: i64,
    #[serde(default = "default_string", rename = "DedeUserID__ckMd5")]
    pub dedeuserid_ckmd5: String,
    #[serde(default = "default_string", rename = "SESSDATA")]
    pub sessdata: String,
    #[serde(default = "default_string", rename = "bili_jct")]
    pub bili_jct: String,
    #[serde(default = "default_i64", rename = "Expires")]
    pub expires: i64,
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
    #[serde(default = "default_i64")]
    pub error_data: i64,
    #[serde(default = "default_string")]
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(default = "default_i64")]
    pub code: i64,
    #[serde(default = "default_string")]
    pub message: String,
    #[serde(default = "default_i64")]
    pub ttl: i64,
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
    pub profession: ProfessionMyInfo,
    pub face_nft: i64,
    pub face_nft_new: i64,
    pub is_senior_member: i64,
    pub level_exp: LevelExp,
    pub coins: f64,
    pub following: i64,
    pub follower: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vip {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub status: i64,
    pub due_date: i64,
    pub vip_pay_type: i64,
    pub theme_type: i64,
    pub label: Label,
    pub avatar_subscript: i64,
    pub nickname_color: String,
    pub role: i64,
    pub avatar_subscript_url: String,
    pub tv_vip_status: i64,
    pub tv_vip_pay_type: i64,
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
pub struct ProfessionMyInfo {
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
    #[serde(default = "default_i64")]
    pub aid: i64,
    #[serde(default = "default_i64")]
    pub videos: i64,
    #[serde(default = "default_i64")]
    pub tid: i64,
    #[serde(default = "default_i64")]
    pub copyright: i64,
    #[serde(default = "default_string")]
    pub pic: String,
    #[serde(default = "default_string")]
    pub title: String,
    #[serde(default = "default_i64")]
    pub ctime: i64,
    #[serde(default = "default_string")]
    pub desc: String,
    #[serde(default = "default_vec", deserialize_with = "null_vec")]
    pub desc_v2: Vec<DescV2>,
    #[serde(default = "default_i64")]
    pub state: i64,
    #[serde(default = "default_i64")]
    pub duration: i64,
    #[serde(default = "default_string")]
    pub dynamic: String,
    #[serde(default = "default_i64")]
    pub cid: i64,
    #[serde(default = "default_dimension")]
    pub dimension: Dimension,
    #[serde(default = "default_bool")]
    pub no_cache: bool,
    #[serde(default = "default_vec", deserialize_with = "null_vec")]
    pub pages: Vec<Page>,
    // todo subtitle...
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescV2 {
    #[serde(default = "default_string")]
    pub raw_text: String,
    #[serde(default = "default_i64", rename = "type")]
    pub desc_type: i64,
    #[serde(default = "default_i64")]
    pub biz_id: i64,
    #[serde(default = "default_rights")]
    pub rights: Rights,
    #[serde(default = "default_owner")]
    pub owner: Owner,
    #[serde(default = "default_stat")]
    pub stat: Stat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rights {
    #[serde(default = "default_i64")]
    bp: i64,
    #[serde(default = "default_i64")]
    elec: i64,
    #[serde(default = "default_i64")]
    download: i64,
    #[serde(default = "default_i64")]
    movie: i64,
    #[serde(default = "default_i64")]
    pay: i64,
    #[serde(default = "default_i64")]
    hd5: i64,
    #[serde(default = "default_i64")]
    no_reprint: i64,
    #[serde(default = "default_i64")]
    autoplay: i64,
    #[serde(default = "default_i64")]
    ugc_pay: i64,
    #[serde(default = "default_i64")]
    is_cooperation: i64,
    #[serde(default = "default_i64")]
    ugc_pay_preview: i64,
    #[serde(default = "default_i64")]
    no_background: i64,
    #[serde(default = "default_i64")]
    clean_mode: i64,
    #[serde(default = "default_i64")]
    is_stein_gate: i64,
    #[serde(default = "default_i64")]
    is_360: i64,
    #[serde(default = "default_i64")]
    no_share: i64,
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
    #[serde(default = "default_i64", rename = "type")]
    pub mid: i64,
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
    #[serde(default = "default_i64")]
    pub aid: i64,
    #[serde(default = "default_i64")]
    pub view: i64,
    #[serde(default = "default_i64")]
    pub danmaku: i64,
    #[serde(default = "default_i64")]
    pub reply: i64,
    #[serde(default = "default_i64")]
    pub favorite: i64,
    #[serde(default = "default_i64")]
    pub coin: i64,
    #[serde(default = "default_i64")]
    pub share: i64,
    #[serde(default = "default_i64")]
    pub now_rank: i64,
    #[serde(default = "default_i64")]
    pub his_rank: i64,
    #[serde(default = "default_i64")]
    pub like: i64,
    #[serde(default = "default_i64")]
    pub dislike: i64,
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
    #[serde(default = "default_i64")]
    pub width: i64,
    #[serde(default = "default_i64")]
    pub height: i64,
    #[serde(default = "default_i64")]
    pub rotate: i64,
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
    #[serde(default = "default_i64")]
    pub cid: i64,
    #[serde(default = "default_i64")]
    pub page: i64,
    #[serde(default = "default_string")]
    pub from: String,
    #[serde(default = "default_string")]
    pub part: String,
    #[serde(default = "default_i64")]
    pub duration: i64,
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
    #[serde(default = "default_i64")]
    pub quality: i64,
    #[serde(default = "default_string")]
    pub format: String,
    #[serde(default = "default_i64")]
    pub timelength: i64,
    #[serde(default = "default_string")]
    pub accept_format: String,
    #[serde(default = "default_vec", deserialize_with = "null_vec")]
    pub accept_description: Vec<String>,
    #[serde(default = "default_vec", deserialize_with = "null_vec")]
    pub accept_quality: Vec<i64>,
    #[serde(default = "default_i64")]
    pub video_codecid: i64,
    #[serde(default = "default_string")]
    pub seek_param: String,
    #[serde(default = "default_string")]
    pub seek_type: String,
    #[serde(default = "default_vec", deserialize_with = "null_vec")]
    pub durl: Vec<Durl>,
    #[serde(default = "default_vec", deserialize_with = "null_vec")]
    pub support_formats: Vec<SupportFormat>,
    #[serde(default = "default_dash")]
    pub dash: Dash,
}

fn default_dash() -> Dash {
    Dash::default()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Durl {
    #[serde(default = "default_i64")]
    pub order: i64,

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

    #[serde(default = "default_vec", deserialize_with = "null_vec")]
    pub backup_url: Vec<String>,
    // todo : highFormat
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportFormat {
    #[serde(default = "default_i64")]
    pub quality: i64,
    #[serde(default = "default_string")]
    pub format: String,
    #[serde(default = "default_string")]
    pub new_description: String,
    #[serde(default = "default_string")]
    pub display_desc: String,
    #[serde(default = "default_string")]
    pub superscript: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dash {
    pub duration: i64,
    #[serde(rename = "minBufferTime")]
    pub min_buffer_time: f64,
    #[serde(rename = "min_buffer_time")]
    pub min_buffer_time2: f64,
    pub video: Vec<Video>,
    pub audio: Vec<Audio>,
    // pub dolby: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub id: i64,
    pub base_url: String,
    #[serde(
        rename = "backupUrl",
        default = "default_vec",
        deserialize_with = "null_vec"
    )]
    pub backup_url: Vec<String>,
    #[serde(
        rename = "backup_url",
        default = "default_vec",
        deserialize_with = "null_vec"
    )]
    pub backup_url2: Vec<String>,
    pub bandwidth: i64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "mime_type")]
    pub mime_type2: String,
    pub codecs: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "frameRate")]
    pub frame_rate: String,
    #[serde(rename = "frame_rate")]
    pub frame_rate2: String,
    pub sar: String,
    #[serde(rename = "startWithSap")]
    pub start_with_sap: i64,
    #[serde(rename = "start_with_sap")]
    pub start_with_sap2: i64,
    pub segment_base: SegmentBase,
    pub codecid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    pub id: i64,
    pub base_url: String,
    #[serde(
        rename = "backupUrl",
        default = "default_vec",
        deserialize_with = "null_vec"
    )]
    pub backup_url: Vec<String>,
    #[serde(
        rename = "backup_url",
        default = "default_vec",
        deserialize_with = "null_vec"
    )]
    pub backup_url2: Vec<String>,
    pub bandwidth: i64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "mime_type")]
    pub mime_type2: String,
    pub codecs: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "frameRate")]
    pub frame_rate: String,
    #[serde(rename = "frame_rate")]
    pub frame_rate2: String,
    pub sar: String,
    #[serde(rename = "startWithSap")]
    pub start_with_sap: i64,
    #[serde(rename = "start_with_sap")]
    pub start_with_sap2: i64,
    pub segment_base: SegmentBase,
    pub codecid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentBase {
    pub initialization: String,
    pub index_range: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SsState {
    // #[serde(rename = "loginInfo")]
    // // pub login_info: LoginInfo,
    // #[serde(rename = "isLogin")]
    // pub is_login: bool,
    // #[serde(rename = "couponSelected")]
    // pub coupon_selected: Value,
    // #[serde(rename = "payGlobal")]
    // pub pay_global: Value,
    // pub loaded: bool,
    // pub ver: Ver,
    // pub ssr: Ssr,
    #[serde(rename = "h1Title")]
    pub h1title: String,
    #[serde(rename = "mediaInfo")]
    pub media_info: MediaInfo,
    // #[serde(rename = "initEpList")]
    // pub init_ep_list: Vec<Ep>,
    #[serde(rename = "epList")]
    pub ep_list: Vec<Ep>,
    #[serde(rename = "epInfo")]
    pub ep_info: Ep,
    // #[serde(rename = "orderSectionIds")]
    // pub order_section_ids: Vec<Value>,
    // pub sections: Vec<Value>,
    // #[serde(rename = "sectionsMap")]
    // pub sections_map: SectionsMap,
    #[serde(rename = "ssList")]
    pub ss_list: Vec<Ss>,
    // #[serde(rename = "userState")]
    // pub user_state: UserState,
    // #[serde(rename = "ssPayMent")]
    // pub ss_pay_ment: SsPayMent,
    // #[serde(rename = "epPayMent")]
    // pub ep_pay_ment: Value,
    // pub player: Player,
    // pub sponsor: Sponsor,
    // #[serde(rename = "ssRecom")]
    // pub ss_recom: SsRecom,
    #[serde(rename = "showBv")]
    pub show_bv: bool,
    // pub interact: Interact,
    // #[serde(rename = "nextEp")]
    // pub next_ep: Value,
    // #[serde(rename = "playerEpList")]
    // pub player_ep_list: PlayerEpList,
    #[serde(rename = "isOriginal")]
    pub is_original: bool,
    #[serde(rename = "premiereCountDown")]
    pub premiere_count_down: String,
    // #[serde(rename = "premiereStatus")]
    // pub premiere_status: PremiereStatus,
    // #[serde(rename = "premiereEp")]
    // pub premiere_ep: PremiereEp,
    // #[serde(rename = "likeMap")]
    // pub like_map: LikeMap,
    // #[serde(rename = "uperMap")]
    // pub uper_map: UperMap,
    #[serde(rename = "hasPlayableEp")]
    pub has_playable_ep: bool,
    // #[serde(rename = "webPlayer")]
    // pub web_player: WebPlayer,
    // #[serde(rename = "epMap")]
    // pub ep_map: Map<String, Ep>,
    #[serde(rename = "viewAngle")]
    pub view_angle: String,
    #[serde(rename = "angleAnimationShow")]
    pub angle_animation_show: bool,
    #[serde(rename = "lastVideoTime")]
    pub last_video_time: i64,
    #[serde(rename = "videoStatus")]
    pub video_status: String,
    #[serde(rename = "insertScripts")]
    pub insert_scripts: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Area {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Publish {
    pub is_finish: i64,
    pub is_started: i64,
    pub pub_time: String,
    pub pub_time_show: String,
    pub unknow_pub_date: i64,
    pub weekday: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rating {
    pub score: f64,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpInfo {
    pub mid: i64,
    pub avatar: String,
    pub name: String,
    #[serde(rename = "isAnnualVip")]
    pub is_annual_vip: bool,
    #[serde(rename = "pendantId")]
    pub pendant_id: i64,
    #[serde(rename = "pendantName")]
    pub pendant_name: String,
    #[serde(rename = "pendantImage")]
    pub pendant_image: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaInfo {
    // pub activity: Activity,
    pub alias: String,
    pub areas: Vec<Area>,
    pub bkg_cover: String,
    pub cover: String,
    pub episodes: Vec<Ep>,
    pub evaluate: String,
    // pub freya: Freya,
    pub jp_title: String,
    pub link: String,
    pub media_id: i64,
    pub mode: i64,
    // pub new_ep: NewEp,
    // pub payment: Payment,
    // pub positive: Positive,
    pub publish: Publish,
    pub rating: Rating,
    pub record: String,
    // pub rights: Rights2,
    pub season_id: i64,
    pub season_title: String,
    // pub seasons: Vec<Ss>,
    pub series: String,
    pub share_copy: String,
    pub share_sub_title: String,
    pub share_url: String,
    // pub show: Show,
    pub square_cover: String,
    // pub stat: Stat2,
    pub status: i64,
    pub subtitle: String,
    pub title: String,
    pub total: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    // pub user_status: UserStatus,
    pub id: i64,
    #[serde(rename = "ssId")]
    pub ss_id: i64,
    #[serde(rename = "jpTitle")]
    pub jp_title2: String,
    #[serde(rename = "ssType")]
    pub ss_type: i64,
    // #[serde(rename = "ssTypeFormat")]
    // pub ss_type_format: SsTypeFormat,
    #[serde(rename = "multiMode")]
    pub multi_mode: bool,
    #[serde(rename = "forceWide")]
    pub force_wide: bool,
    #[serde(rename = "specialCover")]
    pub special_cover: String,
    #[serde(rename = "squareCover")]
    pub square_cover2: String,
    #[serde(rename = "playerRecord")]
    pub player_record: String,
    // #[serde(rename = "pub")]
    // pub pub_field: Pub,
    #[serde(rename = "upInfo")]
    pub up_info: UpInfo,
    // #[serde(rename = "newestEp")]
    // pub newest_ep: NewestEp,
    // #[serde(rename = "payMent")]
    // pub pay_ment: PayMent,
    // #[serde(rename = "payPack")]
    // pub pay_pack: PayPack,
    pub count: Count,
    #[serde(rename = "pgcType")]
    pub pgc_type: String,
    #[serde(rename = "epSpMode")]
    pub ep_sp_mode: bool,
    #[serde(rename = "newEpSpMode")]
    pub new_ep_sp_mode: bool,
    #[serde(rename = "mainSecTitle")]
    pub main_sec_title: String,
    // #[serde(rename = "premiereInfo")]
    // pub premiere_info: PremiereInfo,
    #[serde(rename = "sectionBottomDesc")]
    pub section_bottom_desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Count {
    pub coins: i64,
    pub danmus: i64,
    pub follows: i64,
    pub views: i64,
    pub likes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ep {
    pub aid: i64,
    pub badge: String,
    // pub badge_info: BadgeInfo,
    #[serde(default = "default_i64")]
    pub badge_type: i64,
    pub bvid: String,
    pub cid: i64,
    pub cover: String,
    // pub dimension: Dimension3,
    pub duration: i64,
    pub from: String,
    pub id: i64,
    pub is_view_hide: bool,
    pub link: String,
    pub long_title: String,
    pub pub_time: i64,
    pub pv: i64,
    pub release_date: String,
    // pub rights: Rights4,
    pub share_copy: String,
    pub share_url: String,
    pub short_link: String,
    pub status: i64,
    pub subtitle: String,
    pub title: String,
    pub vid: String,
    // pub skip: Skip2,
    // pub stat: Stat4,
    // #[serde(rename = "orderSectionIds")]
    // pub order_section_ids: Vec<Value>,
    // #[serde(rename = "hasNext")]
    // pub has_next: bool,
    // #[serde(rename = "hasSkip")]
    // pub has_skip: bool,
    #[serde(default = "default_i64")]
    pub i: i64,

    #[serde(rename = "titleFormat", default = "default_string")]
    pub title_format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ss {
    pub id: i64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "pgcType")]
    pub pgc_type: String,
    pub cover: String,
    #[serde(rename = "epCover")]
    pub ep_cover: String,
    pub desc: String,
    pub badge: String,
    #[serde(rename = "badgeType")]
    pub badge_type: i64,
    #[serde(rename = "badgeColor")]
    pub badge_color: String,
    pub views: i64,
    pub follows: i64,
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
    pub code: i64,
}

impl Serialize for VideoQuality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.code.clone())
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
pub const FNVAL_DASH_4K: i64 = 128;
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
    #[serde(default = "default_i64")]
    pub error_data: i64,
    #[serde(default = "default_i64")]
    pub mid: i64,
    #[serde(default = "default_string")]
    pub access_token: String,
    #[serde(default = "default_string")]
    pub refresh_token: String,
    #[serde(default = "default_i64")]
    pub expires_in: i64,
}

/////////

fn default_string() -> String {
    String::default()
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

fn null_vec<'de, D, T: for<'d> serde::Deserialize<'d>>(
    d: D,
) -> std::result::Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(d)?;
    if value.is_null() {
        Ok(vec![])
    } else if value.is_array() {
        let mut vec: Vec<T> = vec![];
        for x in value.as_array().unwrap() {
            vec.push(match serde_json::from_value(x.clone()) {
                Ok(t) => t,
                Err(err) => return Err(serde::de::Error::custom(err.to_string())),
            });
        }
        Ok(vec)
    } else {
        Err(serde::de::Error::custom("type error"))
    }
}

///////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub mid: i64,
    pub name: String,
    pub sex: String,
    pub face: String,
    pub face_nft: i64,
    pub face_nft_type: i64,
    pub sign: String,
    pub rank: i64,
    pub level: i64,
    pub jointime: i64,
    pub moral: i64,
    pub silence: i64,
    pub coins: f64,
    pub fans_badge: bool,
    // pub fans_medal: FansMedal,
    pub official: Official,
    pub vip: Vip,
    pub pendant: Pendant,
    pub nameplate: Nameplate,
    // pub user_honour_info: UserHonourInfo,
    pub is_followed: bool,
    pub top_photo: String,
    pub theme: Theme,
    pub sys_notice: SysNotice,
    // pub live_room: Value,
    pub birthday: String,
    pub school: Option<School>,
    pub profession: ProfessionUserInfo,
    // pub tags: Value,
    pub series: Series,
    pub is_senior_member: i64,
    // pub mcn_info: Value,
    pub gaia_res_type: i64,
    // pub gaia_data: Value,
    pub is_risk: bool,
    pub elec: Elec,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct FansMedal {
//     pub show: bool,
//     pub wear: bool,
//     pub medal: Value,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub path: String,
    pub text: String,
    pub label_theme: String,
    pub text_color: String,
    pub bg_style: i64,
    pub bg_color: String,
    pub border_color: String,
    pub use_img_label: bool,
    pub img_label_uri_hans: String,
    pub img_label_uri_hant: String,
    pub img_label_uri_hans_static: String,
    pub img_label_uri_hant_static: String,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct UserHonourInfo {
//     pub mid: i64,
//     pub colour: Value,
//     pub tags: Vec<Value>,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SysNotice {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct School {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    pub user_upgrade_status: i64,
    pub show_upgrade_window: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elec {
    pub show_info: ShowInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShowInfo {
    pub show: bool,
    pub state: i64,
    pub title: String,
    pub icon: String,
    pub jump_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfessionUserInfo {
    pub name: String,
    pub department: String,
    pub title: String,
    pub is_show: i64,
}

/////////////////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionDetailPage {
    pub aids: Vec<i64>,
    pub archives: Vec<VideoArchive>,
    pub meta: CollectionDetailMeta,
    pub page: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoArchive {
    pub aid: i64,
    pub bvid: String,
    pub ctime: i64,
    pub duration: i64,
    pub interactive_video: bool,
    pub pic: String,
    pub pubdate: i64,
    pub stat: VideoStat,
    pub state: i64,
    pub title: String,
    pub ugc_pay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoStat {
    pub view: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionDetailMeta {
    pub category: i64,
    pub cover: String,
    pub description: String,
    pub mid: i64,
    pub name: String,
    pub ptime: i64,
    pub season_id: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
    pub page_num: i64,
    pub page_size: i64,
    pub total: i64,
}

///////////////////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonsSeriesListData {
    pub items_lists: SeasonsSeriesList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonsSeriesList {
    pub page: PageInfo,
    pub seasons_list: Vec<SeasonsList>,
    pub series_list: Vec<SeriesList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonsList {
    pub archives: Vec<VideoArchive>,
    pub meta: CollectionDetailMeta,
    pub recent_aids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesList {
    pub archives: Vec<VideoArchive>,
    pub meta: SeriesVideoMeta,
    pub recent_aids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Favlist {
    pub mid: i64,
    pub fid: i64,
    pub page_num: i64,
    pub page_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesVideoMeta {
    pub category: i64,
    pub cover: String,
    pub creator: String,
    pub ctime: i64,
    pub description: String,
    pub keywords: Vec<String>,
    pub last_update_ts: i64,
    pub mid: i64,
    pub mtime: i64,
    pub name: String,
    pub raw_keywords: String,
    pub series_id: i64,
    pub state: i64,
    pub total: i64,
}

////////////////////

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesVideoInfoData {
    pub meta: SeriesVideoInfoMeta,
    pub recent_aids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesVideoInfoMeta {
    pub series_id: i64,
    pub mid: i64,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub creator: String,
    pub state: i64,
    pub last_update_ts: i64,
    pub total: i64,
    pub ctime: i64,
    pub mtime: i64,
    pub raw_keywords: String,
    pub category: i64,
}
