#!/bin/bash


link_system() {
	if [[ "$PB_SYSTEM_PATH" != "" ]]; then
        echo "Making link to system ($PB_SYSTEM_PATH)"
        rm -f -r ./system
        ln -s $PB_SYSTEM_PATH
	else
        echo "[warn] Link to system not set authomatically. Please make sure if it exists"
	fi
}

if [[ "$(basename -- "$0")" == "configure.sh" ]]; then
	echo "configure.sh must be sourced"
	echo "run by: source ./configure.sh"
else
    echo "Setting env..."
	if [[ "$1" == "--arm" ]]; then
 	    echo "architecture=arm"
		 . ../../env_set.sh >/dev/null
		export QT_PATH=$TOOLCHAIN_PATH/$TOOLCHAIN_PREFIX/sysroot/ebrmain

		export QT_QPA_PLATFORM=pocketbook2
		#export PATH="$PB_SDK_DIR/usr/bin:$PATH"

		export CC=$PB_SDK_DIR/usr/bin/arm-obreey-linux-gnueabi-gcc
		export CXX=$PB_SDK_DIR/usr/bin/arm-obreey-linux-gnueabi-g++
	elif [[ "$1" == "--emulator" ]]; then
 	    echo "architecture=x86 (emulator)"
		export QT_PATH=$PB_SDK_DIR/local/qt5
		export QT_QPA_PLATFORM=pocketbook2
	else
 	    echo "architecture=x86 ($1)"
		if [[ -z "$1" ]]; then
 	    	echo "qt path must be set: source ./configure.sh [qt/path]"
		fi
		export QT_PATH=$1
		export QT_QPA_PLATFORM=xcb
	fi

	export QMAKE=$QT_PATH/bin/qmake
	export QT_INCLUDE_PATH=$QT_PATH/include
	export QT_LIBRARY_PATH=$QT_PATH/lib

	export LD_LIBRARY_PATH=$QT_LIBRARY_PATH:$LD_LIBRARY_PATH

	echo "PATH=$PATH"
	echo "QMAKE=$QMAKE"
	echo "QT_INCLUDE_PATH=$QT_INCLUDE_PATH"
	echo "QT_LIBRARY_PATH=$QT_LIBRARY_PATH"
	echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
	echo "QT_QPA_PLATFORM=$QT_QPA_PLATFORM"
	echo "CC=$CC"
	echo "CXX=$CXX"
fi


ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BUILD_DIR=$ROOT_DIR/target

echo "ROOT_DIR=$ROOT_DIR"
echo "BUILD_DIR=$BUILD_DIR"

link_system
mkdir -p $BUILD_DIR/debug
cd $BUILD_DIR/debug
link_system
mkdir -p $BUILD_DIR/release
cd $BUILD_DIR/release
link_system

cd $ROOT_DIR
