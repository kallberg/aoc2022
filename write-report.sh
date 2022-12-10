#!/bin/env bash

set -e
date -u +"# Advent of Code %Y - Day %d" > release.md
echo >> release.md
echo "\`\`\`plaintext" >> release.md
./target/release/aoc2022 >> release.md
echo "\`\`\`" >> release.md