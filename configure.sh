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
    export QMAKE=$PB_SDK_DIR/local/qt5/bin/qmake
	export QT_INCLUDE_PATH=$PB_SDK_DIR/local/qt5/include
	export QT_LIBRARY_PATH=$PB_SDK_DIR/local/qt5/lib
	export QT_LIBRARY_PATH=$PB_SDK_DIR/local/qt5/lib

	export LD_LIBRARY_PATH=$QT_LIBRARY_PATH:$LD_LIBRARY_PATH
	export QT_QPA_PLATFORM=pocketbook2

	echo "QMAKE=$QMAKE"
	echo "QT_INCLUDE_PATH=$QT_INCLUDE_PATH"
	echo "QT_LIBRARY_PATH=$QT_LIBRARY_PATH"
	echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
	echo "QT_QPA_PLATFORM=$QT_QPA_PLATFORM"
fi


ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BUILD_DIR=$ROOT_DIR/target/debug/

echo "ROOT_DIR=$ROOT_DIR"
echo "BUILD_DIR=$BUILD_DIR"

link_system
mkdir -p $BUILD_DIR
cd $BUILD_DIR
link_system

cd $ROOT_DIR
