#!/bin/bash

# Cycle through a list of commands with the space key, e.g.:
#
#   ./cycle.sh \
#       "vibrate-experiment market" \
#       "vibrate-experiment morse"

if (( $# == 0 )); then
    echo "USAGE: ./cycle.sh \"CMD1\" \"CMD2\" ..."
    exit 1
fi

COMMANDS=("$@")
CYCLE_KEY=" "

i=0

while :
do
    cmd="${COMMANDS[$i]}"
    echo "Running command $i '$cmd'"

    $cmd &
    pid=$!
    trap "kill $pid" EXIT

    while
        IFS="" read -n 1 k
        [ "$k" != "$CYCLE_KEY" ]
    do :; done

    kill $pid
    trap - EXIT

    i=$((i+1))
    if (( i >= ${#COMMANDS[@]} )); then
       i=0
    fi
done
