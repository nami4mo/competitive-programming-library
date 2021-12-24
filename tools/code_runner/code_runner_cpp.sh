#!/bin/bash -eu
set -eu

sh_dir=$(cd $(dirname $0); pwd)
input_file="${sh_dir}/code_runner_input.txt"

dir=$1
filename=$2
exe_file="$3.a.out"

cd $dir # dir
g++ $filename -std=c++17 -I${HOME}/Desktop/pro_con/ -o $exe_file

if [ -s $input_file ]; then
    "./$exe_file" < $input_file
else
    echo "Please input..."
    "./$exe_file"
fi