# Notion markdown sync
A system that aims to sync a markdown file on the file system with a notion document

# Setup notion

## Get auth token
1. Create an internal for your workspace trough integration https://www.notion.so/my-integrations.
2. Get the secret

## Create a root page
1. Create a page in your workspace
2. Add your earlier created integration to the page
3. Get the page id by the url `notion.so/${title}-${page_id}?{query_params}`


# Run

`cargo run -- notion_token ${token} notion_page_id ${page_id} watch_path ${path_to_file} ext ${ext1} ext ${ext2}`

e.g.

`cargo run -- notion_token secret_Loijfs123jhpaw notion_page_id 189fabe13b2a watch_path /home/user/notes ext md ext txt`