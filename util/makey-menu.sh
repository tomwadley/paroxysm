#!/bin/bash

if (( $# == 0 )); then
    echo "USAGE: ./makey-menu.sh path/to/vibrate-experiment"
    echo ""
    echo "A Makey Makey menu interface for switching modes"
    exit 1
fi

BIN=$1

UP="market"
DOWN="morse"
RIGHT="neo"
#LEFT=""

pid=""

while :
do
    while
        mode=""

        IFS="" read -n 1 k
        [ "$k" == "A" ] && mode="$UP"
        [ "$k" == "B" ] && mode="$DOWN"
        [ "$k" == "C" ] && mode="$RIGHT"
#        [ "$k" == "D" ] && mode="$LEFT"
        [ "$k" == " " ] && exit 0 # Space key exits

        [ "$mode" == "" ]
    do :; done

    [ "$pid" != "" ] && kill $pid

    cmd="$BIN $mode"
    echo "Running command '$cmd'"

    $cmd &
    pid=$!
    trap "kill $pid" EXIT
done
