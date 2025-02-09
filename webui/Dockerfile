FROM oven/bun:1.2

RUN apt install tar gzip bash -y

WORKDIR /app
COPY package.json bun.lock ./

RUN bun install

COPY . .

RUN bun run build

ENTRYPOINT [ "/bin/bash", "-c", "PORT=80 bun run start" ]
