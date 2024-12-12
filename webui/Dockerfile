FROM node:18

RUN apt install tar gzip bash -y

RUN npm install -g npm@10

WORKDIR /app
COPY package.json .
COPY package-lock.json .

RUN npm install

COPY . .

RUN npm run build

ENTRYPOINT [ "/bin/bash", "-c", "PORT=80 npm run start" ]
