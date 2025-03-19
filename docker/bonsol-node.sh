#!/bin/bash

if [ "$GPU" != "" ]; then
  /usr/opt/bonsol/bonsol-node-gpu
else
  /usr/opt/bonsol/bonsol-node-cpu
fi
