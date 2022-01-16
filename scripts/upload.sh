set -e

cargo build
git add . && git commit -m "update" && git push