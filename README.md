# Template Repo

This repo is the foundation for creating a new microservice in the yabs project.

## Using this repo
1. Create a new repo on github and select this as the template.
2. Adjust the name of this app in the Cargo.toml and the Dockerfile entry line.

## Creating a release

```sh
cargo release patch/minor/major --execute
````

##  Creating a sql migration

```sh
sqlx migrate add
```

## Update sql scripts for release

This needs to be ran and commited to the repo to allow
for the ci/cd to build with sql.

```shell
cargo sqlx prepare
```

## How to change the proto repo

1. Open `.gitmodules` and change the url to the new repo
2. Change to the folder
    ```shell
    cd proto
    ```
3. Update the remote
    ```shell
    git remote set-url origin url...
    ```
4. Update code
    ```shell
    git pull origin main
    git reset --hard origin/main
    ```