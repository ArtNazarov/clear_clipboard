#!/bin/bash
CHECK_INTERVAL=0.5
CLEAR_AFTER_SECONDS=9
counter=0
last_content=""

while true; do
    sleep $CHECK_INTERVAL
    # Получаем текущее содержимое буфера
    current_content=$(xsel --clipboard --output 2>/dev/null)

    # Если содержимое изменилось, сбрасываем счётчик
    if [ "$current_content" != "$last_content" ]; then
        counter=0
        last_content="$current_content"
    else
        # Если содержимое не менялось, увеличиваем счётчик
        counter=$(echo "$counter + $CHECK_INTERVAL" | bc -l)
    fi

    # Если прошло больше нужного времени, очищаем буфер
    if (( $(echo "$counter >= $CLEAR_AFTER_SECONDS" | bc -l) )); then
        # run: cargo build --release 
        ./target/release/clear_clipboard -c
        counter=0
        last_content=""
        # echo "Почистили"
    fi
done
