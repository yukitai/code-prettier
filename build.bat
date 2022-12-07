@echo off

cargo build --release --target-dir build
mkdir dist
del /f /s /q .\dist\*
cd .\dist
mkdir highlighting
cd ..
copy .\build\release\prettier.exe .\dist
copy .\highlighting .\dist\highlighting