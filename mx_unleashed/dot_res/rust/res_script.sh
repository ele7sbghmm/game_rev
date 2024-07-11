#!/usr/bin/env bash

function f() {
    if [[ -d "$2" ]]
    then
        # echo if $2
        for file in $2/*
        do f $p/${file:40} $file
        done
    elif [[ -f "$2" ]]
    then if [[ ${2##*.} == "res" ]]
        then cargo run $1 $2
            #  echo $2
        fi
    fi
}

p=/Users/timmeh/Desktop/tracks/
f $p /Users/timmeh/Desktop/mx_psr/Tracks/
