cd ..

cat mp3_sample/sample.mp3 | cargo run > tests/rs-output.txt
cat mp3_sample/sample.mp3 | node codegen_demo.js > tests/node-ouput.txt # redirect output node warning

# diff tests/rs-output.txt tests/node-ouput.txt

cd tests
if ! diff -q rs-output.txt node-ouput.txt &>/dev/null; then
  >&2 echo "Files are different!"
fi

