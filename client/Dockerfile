FROM node:18-alpine3.17 AS builder

ARG NODE_ENV=development

ENV NODE_ENV=$NODE_ENV

WORKDIR /app/client

COPY . .

RUN npm ci
RUN if [ "$NODE_ENV" = "prod" ] || [ "$NODE_ENV" = "production" ]; then npm run build; else npm run build:dev; fi


FROM nginx:1.23.3-alpine AS runtime

COPY --from=builder /app/client/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
