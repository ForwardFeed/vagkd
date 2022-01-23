#!/bin/bash

### HELP FUNCTION

help(){
	cat <<EOF
how to use the script
EOF
}

HelpChangingRights(){
	cat <<EOF
Since you do not seems to be able to read in the input file, which is needed for vagk to work.
You can do multiple things to make the software working.

1)You can run it in priviledged mode using sudo for example

2)You can add your user to the input group like this
	sudo usermod -a -G input $(whoami)

3)There might be other solutions but that would involve GUID and SETUID but i don't know enough on how to setup thoses.

Notes: If you find this to be a security concern, it is a real one. Basically vagk act like a keylogger, it will not log anything but will listen to your keystrokes.
EOF
}

### FUNCTIONS

FindKeyboards()
{
	kbs=$(ls /dev/input/by-id/ | grep Keyboard)
	for kb in $kbs
	do
		ListenKeyboards $kb &
	done
	wait
}

ListenKeyboards()
{
	[ -z $1 ] && exit
	dd if=/dev/input/by-id/${1} status=none bs=24 count=1 >/dev/null
	[ ! $! ] && exit 
	path=$(realpath /dev/input/by-id/"${1}")
	echo "your keyboard seems to be "$path""
	if [ -r $path ]
	then
		echo "and you seems you have the right to read it, perfect"	
	else
		echo "however you do not seems to be able to read it"
		HelpChangingRights
	fi
	echo "you can Ctrl + C now :)"
	exit
}



### RUN IT

case $1 in

	*)
		echo "if nothing is showing, press some keys in your keyboard pleaaase"
		FindKeyboards
	;;

esac
