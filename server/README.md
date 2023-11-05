# install diesel cli in Windows
* Get mysql connector from: https://downloads.mysql.com/archives/c-c/
* Get postgress from: https://www.enterprisedb.com/downloads/postgres-postgresql-downloads
* Get Sqlite from: https://sqlite.org/download.html

copy the lib into: ` C:<where you installed rust>\lib\rustlib\x86_64-pc-windows-gnu\lib`

and then install
```shell
#  keep only what you want: "sqlite-bundled postgres mysql"
cargo install diesel_cli --no-default-features --features "postgres"
```

## ref
* [Linking on Windows](https://github.com/diesel-rs/diesel/issues/587)
* [Installing diesel_cli on Windows : some assembly required](https://github.com/diesel-rs/diesel/issues/487)

