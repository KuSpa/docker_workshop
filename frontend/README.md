# Frontend: Fancy TODO App

This folder contains a simple react frontend. 

### Run locally
Just run
```
npm run start
```
The frontend is then served under `localhost:3000`.

### Run in Docker
In order to run this frontend in Docker, you will to build it first:
```
docker build -t frontend .
```
This creates a docker image containing an NGINX that serves the generated files. See the [Dockerfile](Dockerfile) for more information

To run the docker image, use:
```
docker run -p 80:80 -d frontend
```
The frontend will be served under `localhost:80`. Note, that the port needs to be mapped (`-p 80:80`) and it is not available under port 3000 anymore, since now NGINX is serving the frontend