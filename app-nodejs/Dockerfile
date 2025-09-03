FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

# Copy words.txt from the data directory (already copied by COPY . .)
# This line is redundant if data/words.txt is at the root of the context
# COPY data/words.txt ./data/words.txt

EXPOSE 8080

CMD ["node", "src/api.js"]
