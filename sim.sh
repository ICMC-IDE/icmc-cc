mkdir -p ./target
cargo run $1 > target/out.asm
bin/mnt target/out.asm target/out.mif && bin/sim target/out.mif bin/charmap.mif
