FROM oven/bun:1.1

RUN apt install tar gzip bash -y

RUN bun install -g npm@10

WORKDIR /app
COPY package.json bun.lock ./

RUN bun install

COPY . .

RUN bun run build

ENTRYPOINT [ "/bin/bash", "-c", "PORT=80 bun run start" ]
