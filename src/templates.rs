#[derive(Debug)]
pub struct Template<'a> {
    pub status: &'a str,
    pub timeout: &'a str,
    pub threshold: &'a str,
}

static ZH: Template = Template {
    status: "",
    timeout: "缓存过期时间已设为 {timeout} 秒",
    threshold: "",
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