#!/bin/bash

docker run --rm -it -v ${PWD}:/workspace -w /workspace openssl:alpine-3.10 sh pki.sh