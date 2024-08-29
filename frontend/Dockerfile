# syntax=docker/dockerfile:1.4
FROM node:lts AS builder

WORKDIR /code
COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json

RUN npm ci
COPY . /code
RUN npm run build

# Serving Image
FROM nginx:1.13-alpine

# Copy sources 
COPY --from=builder /code/build /usr/share/nginx/html

EXPOSE 80

# Run nginx in foreground, s.t. docker does not to exit immediately
CMD ["nginx", "-g", "daemon off;"]