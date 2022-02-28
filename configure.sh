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
	if [[ "$2" == "--arm" ]]; then
 	    echo "architecture=arm"
		 . ../../env_set.sh >/dev/null
		export QT_PATH=$TOOLCHAIN_PATH/$TOOLCHAIN_PREFIX/sysroot/usr/qt5
	else
 	    echo "architecture=x86"
		export QT_PATH=$PB_SDK_DIR/local/qt5
	fi

	export QMAKE=$QT_PATH/bin/qmake
	export QT_INCLUDE_PATH=$QT_PATH/include
	export QT_LIBRARY_PATH=$QT_PATH/lib

	export LD_LIBRARY_PATH=$QT_LIBRARY_PATH:$LD_LIBRARY_PATH
	export QT_QPA_PLATFORM=pocketbook2

	echo "QMAKE=$QMAKE"
	echo "QT_INCLUDE_PATH=$QT_INCLUDE_PATH"
	echo "QT_LIBRARY_PATH=$QT_LIBRARY_PATH"
	echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
	echo "QT_QPA_PLATFORM=$QT_QPA_PLATFORM"
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
