#!/bin/bash

d=$(dirname $(realpath $0))
day=$(cat "$0/day")

(( day++ ))

echo day > "$0/inputs/$day"
