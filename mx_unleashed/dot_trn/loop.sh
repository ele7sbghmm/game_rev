#!/usr/bin/env bash

f () {
    if [[ -f "$1" ]]; then
        cargo run $1
    if [[ -d "$1" ]]; then
        for file in $1/*; do
            f $file
        done;
    fi;

f $1
