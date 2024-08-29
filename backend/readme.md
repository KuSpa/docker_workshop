# Backend: Fancy TODO App

This folder contains a simple rust actix backend. 

### Run locally
Just run
```
cargo run 
```
And see no backend spinning up. Why? Because you probably don't have cargo installed (Shame).

### Run with docker
Run 
```
docker build -t backend .
```
to build.

Run
```
docker run -p 8000:8000 backend
```
to run.