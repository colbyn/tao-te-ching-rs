set -e

cargo run
git add . && git commit -m "update" && git push