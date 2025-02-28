#!/bin/bash

get_time_of_day() {
    hour=$1
    if (( hour >= 6 && hour < 12 )); then echo "утро"
    elif (( hour >= 12 && hour < 18 )); then echo "день"
    elif (( hour >= 18 && hour < 23 )); then echo "вечер"
    else echo "ночь"
    fi
}

read -p "Введите ваше имя: " name

while :; do
    read -p "Введите текущее время (0-23): " hour
    if [[ "$hour" =~ ^[0-9]+$ ]] && (( hour >= 0 && hour < 24 )); then
        break
    fi
    echo "Ошибка! Введите число от 0 до 23."
done

while :; do
    read -p "Сколько раз поприветствовать? (максимум 10): " repeat
    if [[ "$repeat" =~ ^[0-9]+$ ]] && (( repeat >= 1 && repeat <= 10 )); then
        break
    fi
    echo "Ошибка! Введите число от 1 до 10."
done

time_of_day=$(get_time_of_day "$hour")

for (( i=1; i<=repeat; i++ )); do
    echo -e "\nПривет, $name! Сейчас $time_of_day. Хорошего дня!"
    sleep 1
    printf '.%.0s' $(seq 1 $i)
    echo
done
