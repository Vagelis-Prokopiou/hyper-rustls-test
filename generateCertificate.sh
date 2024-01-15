#!/bin/env bash

openssl req -x509 -nodes -days 36500 -newkey rsa:2048 -keyout certs/server.key -out certs/server.crt
