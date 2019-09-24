# Mastering rustls

A playground demonstrates usage of [rustls](https://mesatee.org/doc/rustls/index.html) 

## Prerequisite

1. Pack a docker image bundle with openssl
    ```bash
    cd pki/docker 
    # the image would be named as openssl:alpine-3.10
    docker build -t openssl:alpine-3.10
    ```
2. Build PKI system
    ```bash
    # generate
    #   1. cert (self-signed) and key for CA, respectively as ca.cert and ca.key
    #   2. cert (signed by ca.key) and key for server, respectively as server.cert and server.key
    #   3. cert (signed by ca.key) and key for client, respectively as client.cert and client.key
    docker run --rm -it -v ${PWD}:/workspace -w /workspace openssl:alpine-3.10 sh pki.sh
    ```

## Run 
Runnable examples are 
- dangerous-client
- mutual-auth
- simple

Each example has the corresponding client and server, so just change into the 
corresponding directory and bootstrap them with `cargo run` command. 

## Head ups
- key length should be at least 1024