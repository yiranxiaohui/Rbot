#!/bin/bash

# 定义变量
Rbot_path="./bin/Rbot"
log_file="Rbot.log"

# 检查 Rbot 是否在运行
pid=$(pgrep -f "$Rbot_path")

if [ -z "$pid" ]; then
    echo "Rbot is not running."
    exit 0
fi

# 终止 Rbot 进程
echo "Stopping Rbot with PID: $pid"
kill $pid

# 检查是否成功终止
if [ $? -eq 0 ]; then
    echo "Rbot stopped successfully."
else
    echo "Failed to stop Rbot. Check the log file: $log_file"
fi