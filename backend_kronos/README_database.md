# Database Operations

This project uses the Rust crate `SeaORM`. SeaORM is an extension of SQLx, which natively supports async operations and most large databases. SeaORM supports Postgres, which we are usinh. 

## Database Setup

We are working with a fresh database, meaning we want to track migrations. The SeaORM CLI tool helps with this. It can be installed with `cargo install sea-orm-cli`. Note that this will install the binary to help us work with SeaORM, it won't add it as a dependency. 

Our database was created in the standard way. This should be done in the backend_kronos directory.
`sea-orm-cli migrate init`
I referenced this documentation in creating it: https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/


