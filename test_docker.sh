#!/bin/sh

docker build -t openvpn_client_rs .

docker run --rm \
	--name openvpn_client_rs \
	--cap-add=NET_ADMIN \
	--device /dev/net/tun \
	--env-file .env \
 openvpn_client_rs
