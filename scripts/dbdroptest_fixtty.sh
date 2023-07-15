#!/bin/bash
if [ -t 1 ]; then
  # stdout is a terminal
  "$@"
else
  # stdout is a file or pipe
  PROG=$1 ; shift
  set -- "$PROG.exe" "$@" # append .exe suffix
  "$@"
fi