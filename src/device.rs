use serde::Serialize;

#[derive(Serialize)]
pub struct Device {
    pub instagram_version: String,
    pub android_version: String,
    pub android_release: String,
    pub dpi: String,
    pub resolution: String,
    pub manufacturer: String,
    pub device: String,
    pub model: String,
    pub cpu: String,
}

pub fn get_device() -> Device {
    Device {
        instagram_version: String::from("26.0.0.10.86"),
        android_version: String::from("24"),
        android_release: String::from("7.0"),
        dpi: String::from("640dpi"),
        resolution: String::from("1440x2560"),
        manufacturer: String::from("HUAWEI"),
        device: String::from("LON-L29"),
        model: String::from("HWLON"),
        cpu: String::from("hi3660"),
    }
}
