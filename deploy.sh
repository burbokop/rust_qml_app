#!/bin/bash


call_remote() {
    echo -e "\e[101m$1\e[0m"
    ssh root@192.168.205.1 "$1"
}

SCRIPT_FILE=`readlink -f ${0}`
PROJECT_DIR=`dirname ${SCRIPT_FILE}`

echo "/lib/modules/ins_usbnet.sh" | xclip
echo "note: if script hangs you need run '/lib/modules/ins_usbnet.sh' on dst device (copied to clipboard)"

call_remote "mount -o rw,remount /ebrmain"
(cd target/armv7-unknown-linux-gnueabi/debug && tar cv rust_qml_app) | ssh root@192.168.205.1 "tar xv -C /ebrmain/bin"
call_remote "ln -sf /ebrmain/bin/rust_qml_app /ebrmain/bin/rust_qml_app.app"
call_remote "killall -9 rust_qml_app.app"








