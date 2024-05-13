#!/bin/sh

bun run scripts/write.ts
bun run scripts/types.ts
for i in stats handlers
do
    cargo typify ./schema/$i.schema.json -o rs/$i.rs
done
