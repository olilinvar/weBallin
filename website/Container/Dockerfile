FROM nginx:latest

RUN apt-get update \
    && apt-get install -y git \
    && apt-get install -y gnupg \
    && apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv 9DA31620334BD75D9DCB49F368818C72E52529D4 \
    && echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu focal/mongodb-org/4.4 multiverse" | tee /etc/apt/sources.list.d/mongodb-org-4.4.list \
    && apt-get update \
    && apt-get install -y mongodb-org \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 80
EXPOSE 27017

RUN git clone --depth 1 https://github.com/olilinvar/weBallin/website.git

WORKDIR /website

CMD service mongod start && nginx -g "daemon off;"
