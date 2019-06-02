#!/bin/sh
RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/koute/cargo-web/releases/latest | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/' | cut -c 1-)
wget https://github.com/koute/cargo-web/releases/download/${RELEASE}/cargo-web-x86_64-unknown-linux-gnu.gz
gunzip cargo-web-x86_64-unknown-linux-gnu.gz
chmod +x cargo-web-x86_64-unknown-linux-gnu
sudo mv cargo-web-x86_64-unknown-linux-gnu /usr/bin/cargo-web
