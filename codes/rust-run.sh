#!/bin/sh

file=$1

filename=${file%.rs}
pathname=${file%/*}

cd $pathname

echo "格式化代码"
rustfmt $file
echo "rustfmt ${file}";

echo "\n编译代码"
rustc $file
echo "rustc ${file}";

if [ $? -eq 0 ]; then
    echo "\n编译成功, 准备运行"
    echo "------------------------------------"
    ./${filename}
    rm $filename
else 
    echo "------------------------------------"
    echo "编译时出错!!!"
fi
