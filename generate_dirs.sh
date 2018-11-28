#!/bin/bash

for i in $(seq -w 01 25)
do
	dir_name='day'$i

	# if you want to use a specific structure within each folder, you can
	# customize this here and re-run this script
	mkdir -p $dir_name
	touch $dir_name/.keep
done