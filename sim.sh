cargo build &&
target/debug/icmc-cc -i $1 -o target/out.asm &&
cat std/*.asm >> target/out.asm &&
bin/mnt target/out.asm target/out.mif && bin/sim target/out.mif bin/charmap.mif
