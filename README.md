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
