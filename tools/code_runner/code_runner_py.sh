sh_dir=$(cd $(dirname $0); pwd)
input_file="${sh_dir}/code_runner_input.txt"

if [ -s $input_file ]; then
    cat $input_file
    python3 -u $1 < $input_file
else
    echo "Please input..."
    python3 -u $1
fi