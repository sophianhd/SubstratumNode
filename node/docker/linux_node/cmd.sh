#!/bin/bash -xev

sudo cp /tmp/resolv.conf /etc/resolv.conf
sudo /node/node --dns_servers 8.8.8.8 &
sudo dbus-daemon --system --fork
/usr/bin/firefox
