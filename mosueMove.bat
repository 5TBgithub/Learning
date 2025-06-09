@echo off
echo Run Python script mouse move...

REM cai dat pyautogui
python -m pip install pyautogui

REM chay Python script
python -c "import pyautogui, random, time; \
total_duration = 120 * 60; time_interval = 10; \
start_time = time.time(); print('moussemove  Ctrl + C to stop.'); \
try: \
    while time.time() - start_time < total_duration: \
        screen_width, screen_height = pyautogui.size(); \
        pyautogui.moveTo(random.randint(0, screen_width-1), random.randint(0, screen_height-1), duration=0.5); \
        time.sleep(time_interval); \
except KeyboardInterrupt: \
    print('\\nScript dung theo yeu cau.'); print('Complete!')" 

echo Script ket thuc.
pause