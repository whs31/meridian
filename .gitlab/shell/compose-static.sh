#!/bin/bash

ls -la &&
mkdir "Windows" &&
mkdir "Linux" &&
cd Windows &&
mkdir "x64" &&
cd ../Linux &&
mkdir "x64" &&
cd .. &&
cp libmeridian_win.a ./Windows/x64/libmeridian.a &&
cp libmeridian.a ./Linux/x64/libmeridian.a &&
ls -la .