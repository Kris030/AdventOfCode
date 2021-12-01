#!/bin/bash

d=$(dirname $(realpath $0))
day=$(cat "$0/$day")

cat "$0/inputs/$day"
