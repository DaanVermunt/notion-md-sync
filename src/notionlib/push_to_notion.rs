use std::convert::identity;
use std::io::SeekFrom::Start;
use std::path::PathBuf;
use notion::{Error, NotionApi};
use notion::ids::{DatabaseId};
use notion::models::CodeLanguage::Dart;
use notion::models::properties::PropertyValue;
use notion::models::search::{DatabaseQuery, FilterCondition};
use notion::models::search::PropertyCondition::RichText;
use notion::models::search::TextCondition::{Contains, Equals};
use crate::conf::conf;
use crate::notionlib::get_or_create_database::get_or_create_database;

pub struct NotionSyncer {
    pub database_id: DatabaseId,
    token: String,
}

impl Clone for NotionSyncer {
    fn clone(&self) -> Self {
        return NotionSyncer {
            database_id: self.database_id.clone(),
            token: self.token.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.database_id = source.database_id.clone();
        self.token = source.token.clone();
    }
}


impl NotionSyncer {
    pub async fn new(token: String) -> NotionSyncer {
        let database_id = get_or_create_database().await;

        if database_id.is_err() {
            panic!("Could not create database");
        }

        let id: DatabaseId = database_id.unwrap().parse().unwrap();
        NotionSyncer {
            database_id: id,
            token: conf().token,
        }
    }


    async fn get_page_id(self, path: String) -> Option<String> {
        let filter = FilterCondition {
            property: "path".to_string(),
            condition: RichText(Equals(path)),
        };
        let query = DatabaseQuery {
            sorts: None,
            filter: Some(filter),
            paging: None,
        };

        let client =NotionApi::new(self.token).unwrap();
        let pages = client
            .query_database(self.database_id, query)
            .await
            .unwrap()
            .results;

        if pages.len() == 0 {
            return None;
        }

        let page_id = pages.get(0).unwrap().properties.properties.get("page_id").unwrap();

        match page_id {
            PropertyValue::Text { rich_text, id } => {
                Some(rich_text.get(0).unwrap().plain_text().to_string())
            }
            _ => None
        }
    }

    pub async fn push_file_to_notion(self, path: PathBuf) -> Result<(), Error> {
        let page_id = self.get_page_id(String::from(path.to_str().unwrap())).await;

        match page_id {
            Some(page_id) => {
                println!("yes it does {:?}", page_id);
                Ok(())
            }
            None => {
                println!("no it does not");
                Ok(())
            }
        }
    }
}

