FROM node:lts-alpine3.14 AS builder

RUN npm install -g minify

COPY src/favicon.png /build/favicon.png
COPY src/github-icon.png /build/github-icon.png
COPY src/logo.svg /build/logo.svg

COPY src/index.html /src/
COPY src/style.css /src/

WORKDIR /build

RUN minify /src/index.html > /build/index.html
RUN minify /src/style.css > /build/style.css

FROM nginx:stable-alpine

COPY --from=builder /build /usr/share/nginx/html

