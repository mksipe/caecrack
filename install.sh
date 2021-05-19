#!/usr/bin/env sh

if [ $(/usr/bin/id -u) -ne 0 ] ; then
    printf "You must be root.\n"
    exit
elif [ $(/usr/bin/id -u) -eq 0 ]; then
    if [ $(which cargo > /dev/null ; echo $?) -ne 0 ]; then
        printf "Cargo is not installed.\n"
    elif [ $(which cargo > /dev/nulll; echo $?) -eq 0 ]; then
        printf "Making installation.\n"
        printf "Compiling binaries...\n" 
        cargo build --release
        cp target/debug/caecrack /usr/bin/ > /dev/null
       	if [ $(which caecrack > /dev/null; echo $?) -eq 0 ]; then
		printf "`caecrack -V`, has been installed.\n"
	elif [ $(which caecrack > /dev/null; echo $?) -ne 0 ]; then
		printf "caecrack could not be installed.\n"
	fi
    fi
fi
