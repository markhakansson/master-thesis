#!/bin/sh
if ! command -v latexmk &> /dev/null
then
    echo "latexmk could not be found"
    exit 1
fi

latexmk -pvc -pdf -outdir=out
exit 0
