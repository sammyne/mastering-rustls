#!/bin/sh

openssl req -x509 -newkey rsa:2048 \
    -out ca.cert \
    -keyout ca.key \
    -subj /C=CN/ST=SH/O=ORG \
    -days 30 \
    -batch -nodes

openssl genrsa -out server.key 2048
openssl req -new -key server.key \
    -out server.csr \
    -subj /C=CN

openssl x509 -req \
    -in server.csr \
    -out server.cert \
    -CA ca.cert \
    -CAkey ca.key \
    -sha256 \
    -days 30 \
    -set_serial 456 \
    -extensions v3_server \
    -extfile openssl.conf