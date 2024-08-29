# Fancy TODO App

This repository contains a basic TODO App to showcase interaction between components and docker as simple "Glue-it-all-together" tool.

 See [backend/README.md](backend/README.md) for the backend specifics and [frontend/README.md](frontend/README.md) for -- well -- the frontend specifics.

 For the *glue* part see [compose.yaml](compose.yaml).

 ### Run the app ()
 Run 

 ```
 docker compose up
 ```
 which will spin up every both frontend and backend as well as a DB. 

 The first time everything is spun up, the backend will crash. This is intended and to showcase dependencies. Remember that we (will) talk(ed) about this?

 On second start, there will be no more crashes