# Database Operations

This project uses the Rust crate `SeaORM`. SeaORM is an extension of SQLx, which natively supports async operations and most large databases. SeaORM supports Postgres, which we are usinh. 

## Database Setup

We are working with a fresh database, meaning we want to track migrations. The SeaORM CLI tool helps with this. It can be installed with `cargo install sea-orm-cli`. Note that this will install the binary to help us work with SeaORM, it won't add it as a dependency. 

Our database was created in the standard way. This should be done in the backend_kronos directory.
`sea-orm-cli migrate init`
I referenced this documentation in creating it: https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/

Migrations within the database are named by the following convention, and placed in backend_kronos/migrations/src/<file>.rs
'mYYYYMMDD_HHMMSS_migration_name.rs' The exception to this is manually written migrations, where HHMMSS can be replaced by a simple serial (000001, 000002, etc.)

## Generating entities

IMHO, SeaORM explains this only adequately. I took the route of writing migrations first, and from there implying entities. So to make a piece of data accessible at the application level:
(1) Write a migration file, with an up and a down method, in `backend_kronos/migration/src`
(2) Run the migrations against the database from the `backend_kronos/migration` directory. 
- This will alter the actual, real database that you are connected to.
(3) Then, run:
```sh
    # In backend_kronos directory
    sea-orm-cli generate entity \
        -u postgres://postgres:password@localhost:5432/kronos_db \
        -o src/models/entities \
        --with-serde both
```

## Types, and Safety

There are three levels at which to ensure type-safety and data validation (related, but distinct) in the application, i.e., if we want to make sure that a Rank is taken from a set of values defined in an enum, we need to ensure a limited scoping of valid values, and consistent typing. There are three place to do this:
- The frontend (typescript) can have some built-in enums that ensure that rank is rank is rank
- The backend (rust) can do the same
- The database (postgres) has a way to define types as enums

I am opting for the first two options; and in postgres, certain types (rank, for instance) are just going to be stored as strings. As long as only the Rust backend touches the database, this is fine and valid values and valid types are ensured. I would use enums in Postgres but the ORM we're using (SeaORM) despite good reviews is absolutely awful about this, and to define enums within postgres would require rewriting the enum in like three places in just the backend, which naturally defeats the purpose. Maybe SeaORM v2 will change this. 

TLDR: ** The frontend will do input validation, the backend will do typechecking and enumeration, and the database will store strings. **