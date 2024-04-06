Консольный циклический таймер в Rust, который воспроизводит случайный файл .mp3 или .wav из набора аудиосигналов по истечении заданного времени. Звуковые оповещения продолжаются до тех пор, пока их не прервут вручную, введя слово "перерыв" и нажав клавишу ENTER. После прерывания таймер возобновляет работу.

Это будет полезно тем, кто хочет время от времени делать перерывы при работе за компьютером. Время работы таймера в секундах задается в файле config.yml

Вы можете добавить свои собственные звуки в папку /sounds в формате .mp3 или .wav.
-----------------------------------------------------------------------------------
A console cyclic timer in Rust that plays a random .mp3 or .wav from a set of audio signals after a set time has elapsed. The sound alerts continue until they are interrupted manually by entering the word 'break' and pressing the ENTER key. After the interruption, the timer resumes.

It will be useful for those who want to take breaks from time to time when working at a computer. The timer time in seconds is set in the config.yml file

You can add your own sounds to the /sounds folder in .mp3 or .wav format.
