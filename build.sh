#!/bin/sh
mkdir -p rust
cd rust

DOWNLOAD_LINK="https://github.com/rust-lang/rust/tarball/`rustc --version|awk '{sub(/\\(/, "", $3); print $3}'`"

which wget >/dev/null 2>/dev/null
if [ $? -eq 0 ]; then
  wget -q -O rust.tar.gz "$DOWNLOAD_LINK"
else
  which curl >/dev/null 2>/dev/null
  if [ $? -eq 0 ]; then
   curl -L -o rust.tar.gz "$DOWNLOAD_LINK"
  else
    echo "wget or curl required"
    exit 1
  fi
fi
tar -zx --strip-components=1 -f rust.tar.gz
