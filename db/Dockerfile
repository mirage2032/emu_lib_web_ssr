FROM postgres:latest
# Install Dependencies
RUN apt-get -y update && \
    apt-get -y install postgresql-contrib postgresql-plperl-16 make

# Install cpan
RUN echo y | cpan
RUN cpan Email::Address

# Expose Port
EXPOSE 5432