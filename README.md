# Microservice using Rust, Sea Orm and Rocket

## Bootstrap postgres database along with pgAdmin
`podman-compose up`

## Install migration tool
`cargo install sea-orm-cli`

## Verify migration tool setup
`sea-orm-cli --help`

## Initialize migration crate
`sea-orm-cli migrate init`

## Execute migration
`sea-orm-cli migrate -u postgresql://<USERNAME>:<PASSWORD>@localhost:5432/postgres`

## Tables Created Successfully
- post
- seaql_migrations

![](.README_images/b2d6954a.png)

## Reverse generate the entity structs in src/entities directory
`sea-orm-cli generate entity -u postgresql://<USERNAME>:<PASSWORD>@localhost:5432/postgres -o src/entities`

## Get all posts
`curl  localhost:8000/post | jq .`

![](.README_images/70f311ed.png)

## Create a post
`curl  -XPOST 'localhost:8000/post' -H 'Content-Type: application/json' -d '{"text": "Hello World !!","title": "first chapter4..."}' | jq .`

![](.README_images/28cc7d6c.png)


## Update a record
`curl  -XPUT 'localhost:8000/post/3' -H 'Content-Type: application/json' -d '{"text": "a new text for chap4","title": "first chapter4..."}' | jq .`

![](.README_images/b5e87513.png)


## Delete a record
`curl -v -XDELETE localhost:8000/post/3`

![](.README_images/7a4cfbf5.png)