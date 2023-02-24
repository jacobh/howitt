FROM public.ecr.aws/lambda/nodejs:14-arm64

RUN yum install tar gzip bash -y

RUN npm install -g n
RUN n 7

COPY package.json .
COPY package-lock.json .

RUN npm install

COPY . .

RUN npm run build

ENTRYPOINT [ "/bin/bash", "-c", "PORT=80 npm run start" ]