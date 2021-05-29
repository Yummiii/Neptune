#!/bin/bash
local=~/Taiga/Printis/$(date +%Y-%m)/
local=${local}$(($(find ${local}.. -type f | wc -l)+1)).png
flameshot gui -r > ${local}
