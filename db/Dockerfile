FROM postgres:17.5-alpine3.22

RUN apk add --no-cache \
      perl-dev \
      perl-email-address \
      postgresql17-plperl

RUN mkdir -p \
      /usr/local/lib/postgresql \
      /usr/local/share/postgresql/extension \
 && ln -sf /usr/lib/postgresql17/plperl.so \
        /usr/local/lib/postgresql/plperl.so \
 && ln -sf /usr/share/postgresql17/extension/plperl.control \
        /usr/local/share/postgresql/extension/plperl.control \
 && cp /usr/share/postgresql17/extension/plperl--*.sql \
        /usr/local/share/postgresql/extension/

COPY ./initdb /docker-entrypoint-initdb.d

# Expose Postgres port
EXPOSE 5432