#! /usr/bin/env fish

echo "Run any cargo command"

while true
  set path (cat .devcontainer/command-pipe)

  kill %1
  reset

  eval "RUST_BACKTRACE=1 $path &"
end