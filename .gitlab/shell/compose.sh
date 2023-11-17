#!/bin/bash

ls -la &&
mkdir "Windows" &&
mkdir "Linux" &&
cd Windows &&
mkdir "x64" &&
cd ../Linux &&
mkdir "x64" &&
cd .. &&
cp meridian.dll ./Windows/x64/meridian.dll &&
cp libmeridian.so ./Linux/x64/libmeridian.so &&
ls -la .