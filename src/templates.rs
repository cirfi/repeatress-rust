#[derive(Debug)]
pub struct Template<'a> {
    pub status: &'a str,
    pub timeout: &'a str,
    pub threshold: &'a str,
}

static ZH: Template = Template {
    status: "当前会话缓存数：{cache}\n当前触发阈值：{threshold} 次\n当前缓存过期时间：{timeout} 秒",
    timeout: "缓存过期时间已设为 {timeout} 秒",
    threshold: "触发阈值已设为 {threshold} 次",
};

static EN: Template = Template {
    status: "",
    timeout: "The cache expire time is set to {timeout}s",
    threshold: "",
};

pub fn get_template(lang: &str) -> &Template {
    match lang {
        "zh" => &ZH,
        "en" => &EN,
        _ => &ZH
    }
}