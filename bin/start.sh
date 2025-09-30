#!/bin/bash


# 定义变量
Rbot_path="./bin/Rbot"
log_file="Rbot.log"

chmod +777 $Rbot_path

# 检查 Rbot 是否已经在运行
if pgrep -f "$Rbot_path" > /dev/null; then
    echo "Rbot is already running."
    exit 1
fi

# 启动 Rbot 并重定向输出
nohup $Rbot_path > $log_file 2>&1 &

# 检查启动是否成功
if [ $? -eq 0 ]; then
    echo "Rbot started successfully. Log file: $log_file"
else
    echo "Failed to start Rbot. Check the log file: $log_file"
fi