# Simple Frontend Build
FROM node:20-alpine as builder

WORKDIR /app

# Copy and install dependencies
COPY frontend/package*.json ./
RUN npm ci --silent

# Copy source and build
COPY frontend/ .
RUN npm run build

# Serve with nginx
FROM nginx:alpine

# Copy built app
COPY --from=builder /app/dist /usr/share/nginx/html

# Simple nginx config
COPY docker/nginx/default.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]