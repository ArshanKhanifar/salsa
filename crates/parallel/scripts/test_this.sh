#!/bin/bash

export PATH=$PATH:$(pwd)/target/debug

function errorsSometimes() {
  errors=$1
  echo "yoooo starting, errors: $errors"
  while true; do 
    echo "hello - $errors"
    sleep `echo "scale=2; $RANDOM/32768 + 1" | bc -l`
    if [ "$errors" == "true" ]; then
      echo "ERROR: something went wrong" 1>&2
      exit 1
    fi
  done
}

# Export the function so it's available in subshells
export -f errorsSometimes

function main() {
  echo "yo"
  parallel --args "false false true false" --cmd errorsSometimes
}

if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
  main
fi
