#!/bin/bash

sed -i '' 's/PRIVATE KEY/RSA PRIVATE KEY/g' ca.key


cat ca.cert > hello.cert
cat server.cert >> hello.cert

cp server.key ../server/
cp server.cert ../server/server.cert

cp ca.cert ../client/ca.cert