use std::env::args;

pub struct SyncConfig {
    pub extensions: Vec<String>,
    pub path: String,
    pub token: String,
    pub page_id: String,
}

pub fn conf() -> SyncConfig {
    let base = String::from("none");
    let mut conf = SyncConfig {
        path: base.clone(),
        extensions: Vec::from([]),
        token: base.clone(),
        page_id: base.clone(),
    };

    let args: Vec<String> = args().collect();

    for chunk in args[1..].chunks(2) {
        match chunk {
            [key, value] => {
                if key == "watch_path" {
                    conf.path = value.to_owned();
                }

                if key == "ext" {
                    conf.extensions.push(value.to_owned());
                }

                if key == "notion_token" {
                    conf.token = value.to_owned();
                }

                if key == "root_page_id" {
                    conf.page_id = value.to_owned();
                }
            },
            _ => {},
        }

    }

    if conf.extensions.len() == 0 {
        panic!("Extensions not defined, ext")
    }
    if conf.path == base {
        panic!("Path not defined, watch_path")
    }
    if conf.page_id == base {
        panic!("Page id not defined, page_id")
    }
    if conf.token == base {
        panic!("Notion token not defined, notion_token")
    }

    return conf;
}
