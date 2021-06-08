#!/bin/bash
local=~/Taiga/Printis/$(date +%Y-%m)/
[ ! -d ${local} ] && mkdir ${local}
local=${local}$(($(find ${local}.. -type f | wc -l)+1)).png
print=$(flameshot gui -r | base64)
[ "$print" != "c2NyZWVuc2hvdCBhYm9ydGVkCg==" ] && echo "$print" | base64 --decode > "$local"