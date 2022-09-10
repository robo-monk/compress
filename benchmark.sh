echo "compiling to prod"
cargo build -r
echo "done compiling"

echo "\ntiming compress ->"
time ./target/release/compress -i test.big.txt -o test.big.txt.zzz

echo "\ntiming decompress ->"
time ./target/release/compress -i test.big.txt.zzz -o test.big.out.txt -d

rm test.big.out.txt.gz
echo "\ntiming gzip compress ->"
time gzip test.big.out.txt -k

rm test.big.txt
echo "\ntiming decompress ->"
time gzip test.big.txt.gz -k -d


