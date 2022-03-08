cargo doc --release --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=OispaHallaAnticheat\">" > target/doc/index.html
cp -r target/doc ./docs
cargo readme > README.md
