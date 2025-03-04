#!/bin/bash

if [ "$GPU" != "" ]; then
  /usr/opt/bonsol/bonsol-gpu
else
  /usr/opt/bonsol/bonsol-cpu
