cargo build --release --target-dir build
rm -rf ./dist
mkdir -p dist
cd ./dist
mkdir highlighting
cd ..
cp ./build/release/prettier ./dist
cp -r ./highlighting ./dist/highlighting