FROM nginx:1.23.3-alpine AS runtime

ARG SERVER_URL
ENV SERVER_URL=$SERVER_URL

ARG CLIENT_URL
ENV CLIENT_URL=$CLIENT_URL

RUN mkdir /etc/nginx/templates

COPY ./nginx/default.conf.template /etc/nginx/templates
