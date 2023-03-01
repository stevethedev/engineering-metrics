FROM nginx:1.23.3-alpine AS runtime

ARG SERVER_URL
ENV SERVER_URL=$SERVER_URL

ARG CLIENT_URL
ENV CLIENT_URL=$CLIENT_URL

RUN mkdir /etc/nginx/templates
COPY ./infrastructure/nginx/default.conf.template /etc/nginx/templates

WORKDIR /app

## add permissions for nginx user
RUN chown -R nginx:nginx /app && chmod -R 755 /app && \
        chown -R nginx:nginx /var/cache/nginx && \
        chown -R nginx:nginx /var/log/nginx && \
        chown -R nginx:nginx /etc/nginx/conf.d
RUN touch /var/run/nginx.pid && \
        chown -R nginx:nginx /var/run/nginx.pid

USER nginx
