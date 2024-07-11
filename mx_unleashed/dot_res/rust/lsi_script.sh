#!/usr/bin/env bash

function f() {
    if [[ -d "$2" ]]
    then
        # echo if $2
        for file in $2/*
        do f $p/${file:40} $file
        done
    elif [[ -f "$2" ]]
    then if [[ ${2##*.} == "lsi" ]]
        then cargo run $1 $2
            #  echo $2
        fi
    fi
}

p=/tmp/all_lsi
f $p /Users/mair/kode/rev/game_files/mx/xbox/
