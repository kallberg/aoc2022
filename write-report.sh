#!/bin/env bash

set -e
date -u +"# Advent of Code %Y - Day %d" > Release.md
echo >> Release.md
echo "\`\`\`plaintext" >> Release.md
./target/release/aoc2022 >> Release.md
echo "\`\`\`" >> Release.md