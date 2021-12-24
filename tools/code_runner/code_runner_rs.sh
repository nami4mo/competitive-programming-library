#!/bin/bash -eu
set -eu

sh_dir=$(cd $(dirname $0); pwd)
input_file="${sh_dir}/code_runner_input.txt"
rust_runner_dir="${sh_dir}/runner_rs"

RUNNER_MAIN="src/main.rs"
RUNNER_EXE="./target/debug/runner_rs"

dir=$1
filename=$2
filename_without_ext=$3

cd $dir # dir
rm "${rust_runner_dir}/${RUNNER_MAIN}" || true
cp $filename "${rust_runner_dir}/${RUNNER_MAIN}"
cd $rust_runner_dir
cargo build

if [ -s $input_file ]; then
    $RUNNER_EXE < $input_file
else
    echo "Please input..."
    $RUNNER_EXE
fi