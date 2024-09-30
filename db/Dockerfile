FROM postgres:latest
RUN apt-get -y update && \
    apt-get -y install postgresql-contrib postgresql-plperl-16 make

RUN echo y | cpan
RUN cpan Email::Address