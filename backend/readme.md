# Backend: Fancy TODO App

This folder contains a simple rust actix backend. 

### Run locally
Just run
```
cargo run 
```
And see no backend spinning up. Why? Because you probably don't have cargo installed. Even if, the error message hints that you need some db-configuration like a dbName to run the backend. Thus, it makes little sense to run this backend outside of the larger context.

See [../compose.yaml](../compose.yaml) to see what config is required.