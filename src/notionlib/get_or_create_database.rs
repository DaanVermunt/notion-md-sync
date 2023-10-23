use notion::{Error, NotionApi};
use notion::models::{ListResponse, Object};
use notion::models::search::{NotionSearch, SearchRequest};
use regex::Regex;
use crate::conf::conf;
use crate::notionlib::{DATABASE_NAME, NOTION_URL};

async fn create_database() -> Result<String, Error> {
    println!("Creating database...");
    let body = format!("{{
        \"parent\": {{
            \"type\": \"page_id\",
            \"page_id\": \"1eb4f2a6090249cda16f343fcbe4351c\"
        }},
        \"icon\": {{
            \"type\": \"emoji\",
            \"emoji\": \"🛢️\"
        }},
        \"title\": [
        {{
            \"type\": \"text\",
            \"text\": {{
            \"content\": \"{DATABASE_NAME}\"
        }}
        }}
        ],
        \"properties\": {{
            \"Name\": {{
                \"title\": {{}}
            }},
            \"path\": {{
                \"rich_text\": {{}}
            }},
            \"page_id\": {{
                \"rich_text\": {{}}
            }},
            \"last_edit\": {{
                \"date\": {{}}
            }}
        }}
    }}", );

    let url = NOTION_URL.to_owned() + "/databases";
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .body(body)
        .header("Authorization", format!("Bearer {}", conf().token))
        .header("Notion-Version", "2022-06-28")
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;


    let re = Regex::new(r#""id":"(.*?)""#).unwrap();
    let caps = re.captures(&res).unwrap();
    let id = caps.get(0).unwrap().as_str().replace("\"id\":", "").replace("\"", "");

    print!("Created database with id: {}", id);
    return Ok(String::from(id));
}

pub async fn get_or_create_database() -> Result<String, Error> {
    let query = SearchRequest::from(NotionSearch::Query(DATABASE_NAME.to_string()));

    let res: ListResponse<Object> = NotionApi::new(conf().token)?.search(query).await?;
    // TODO GET AWAY FROM ASSUMPTION OF FIRST ITEM BEING THE DB
    let database = res.results.iter().next();

    let res = match database {
        Some(database) => {
            match database {
                Object::Database { database } => database.id.to_string(),
                _ => create_database().await?,
            }
        }
        None => create_database().await?,
    };
    return Ok(res)
}