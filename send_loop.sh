#!/bin/bash
while true
do
	echo LAST SEND $(date)
	scp ./script.lua ./values.lua robot:~/
	echo "PRESS TO SEND"
	read
done
