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
COPY --from=builder from to/oh/nein/meine/katze/ist/ueber/meine/tastatur/gelaufen/

# Erlaubt Docker mit diesem Port zu interagieren
EXPOSE 42


# Startet NGINX. Mit "-g" "deamon off;" startet NGINX im Vordergrund. Aber das brauchen wir ja bestimmt nicht. Right? RIGHT?
CMD ["nginx"]
# vs CMD ["nginx", "-g", "daemon off;"]