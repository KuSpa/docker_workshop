# Backend: Fancy TODO App

This folder contains a simple rust actix backend. 

### Run locally
Just run
```
cargo run 
```
And see no backend spinning up. Why? Because you probably don't have cargo installed. Even if, the error message hints that you need some db-configuration like a dbName to run the backend. Thus, it makes little sense to run this backend outside of the larger context.

The following environment variables are required:
* PG_HOST (address of the docker running the db)
* PG_DBNAME (should be 'postgres')
* PG_USER (should be 'postgres')
* PG_PASSWORD (should not be written in plain text due to security reasons ;). However rumor has it that the postgres docker has a similar setting where you can look)