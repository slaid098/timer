# Консольный циклический таймер на Rust

Этот таймер предназначен для воспроизведения случайного звукового файла (.mp3 или .wav) из заданного набора аудиосигналов по истечении установленного времени. Звуковые уведомления будут повторяться, пока пользователь не прервёт их, введя команду "break" и нажав клавишу ENTER. После этого таймер автоматически возобновляет свою работу.

## Применение

Данный таймер будет особенно полезен для тех, кто хочет регулярно делать перерывы во время работы за компьютером, способствуя сохранению высокой производительности и заботе о здоровье.

## Настройка времени

Длительность таймера (в секундах) настраивается в файле `config.toml`, что позволяет пользователям гибко подстроить таймер под индивидуальные потребности.

## Кастомизация звуковых уведомлений

Вы можете добавить собственные аудиофайлы в папку `/sounds`, используя форматы .mp3 или .wav, чтобы персонализировать звуковые уведомления таймера.
