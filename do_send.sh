ping 10.42.0.3 -c 1 -w 1 || exit 1;
luac -p ./rrr/values.lua ./rrr/script.lua || exit 1;
scp ./rrr/values.lua ./rrr/script.lua robot:~/
paplay /usr/share/sounds/gnome/default/alerts/glass.ogg &
