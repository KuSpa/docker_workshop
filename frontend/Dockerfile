# syntax=docker/dockerfile:1.4
# Base image mit installiertem node/npm
FROM node:lts AS builder

WORKDIR /code
COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json

# Installiere Abhängigkeiten
RUN npm ci
COPY . /code
# Baut einen Ordner in ./build der statisch deployed werden kann
RUN npm run build

# Serving Image
# NGINX deployed per se alles was in /usr/share/nginx/html liegt unter port 80
FROM nginx:1.13-alpine

# Kopiert react app in den html ordner, sodass NGINX ihn findet
COPY --from=builder /code/build /usr/share/nginx/html

# Erlaubt Docker mit diesem Port zu interagieren
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]