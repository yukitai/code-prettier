@echo off

cargo build --release --target-dir build
mkdir dist
cd .\dist
mkdir highlighting
cd ..
del /f /s /q .\dist\*.*
copy .\build\release\prettier.exe .\dist
copy .\highlighting .\dist\highlighting