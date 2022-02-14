use crypto::digest::Digest;

use crate::types::*;

//////////////////// JSON ////////////////////

/// FROM STRING 并打印出错的位置
pub fn from_str<T: for<'de> serde::Deserialize<'de>>(json: &str) -> Result<T> {
    Ok(serde_path_to_error::deserialize(
        &mut serde_json::Deserializer::from_str(json),
    )?)
}

//////////////////// AV->BV ////////////////////

const AV_TABLE: &'static str = "fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const XOR: i64 = 177451812;
const ADD: i64 = 100618342136696320;
const S: [usize; 10] = [9, 8, 1, 6, 2, 4, 0, 7, 3, 5];

pub fn av_to_bv(avid: i64) -> String {
    let table = AV_TABLE.as_bytes();
    let x1 = (avid ^ XOR) + ADD;
    let mut pow = 1;
    let mut bytes = vec![0; 10];
    for i in 0..10 {
        let index = x1 / pow % 58;
        bytes[S[i]] = table[index as usize];
        pow *= 58;
    }
    format!("BV{}", String::from_utf8(bytes).unwrap())
}

////////////////////////////////////////////////

///////////////////// SIGN /////////////////////

pub fn sign_form(params: serde_json::Value, app_sec: &str) -> Result<serde_json::Value> {
    // 获取签名参数的对象
    if !params.is_object() {
        return Err(Box::new(Error::from("sign error: not object")));
    }
    let obj = params.as_object().unwrap();
    // 获取Keys并排序
    let mut key_list: Vec<String> = vec![];
    for (key, _value) in obj {
        key_list.push(key.clone());
    }
    key_list.sort();
    // 遍历key
    let mut link_list: Vec<String> = vec![];
    for key in key_list {
        let item = obj.get(&key).unwrap();
        if item.is_string() {
            link_list.push(format!("{}={}", key, item.as_str().unwrap()))
        } else {
            link_list.push(format!("{}={}", key, item.to_string()))
        }
    }
    let link_list = format!("{}{}", link_list.join("&"), app_sec);
    let mut hasher = crypto::md5::Md5::new();
    hasher.input_str(&link_list);
    let mut obj = obj.clone();
    obj.insert(
        "sign".to_string(),
        serde_json::to_value(hasher.result_str())?,
    );
    Ok(serde_json::Value::Object(obj))
}

////////////////////////////////////////////////
