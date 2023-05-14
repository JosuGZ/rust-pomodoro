set -o errexit
set -o nounset
set -o pipefail

cross build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/omad TARGET
