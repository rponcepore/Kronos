This directory (entities) holds the SeaORM generated entity files. 
It was created with the command:

```sh
# In backend_kronos directory
    sea-orm-cli generate entity \
        -u postgres://postgres:password@localhost:5432/kronos_db \
        -o src/models/entities \
        --with-serde both
```