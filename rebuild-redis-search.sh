 # 1. Clear previous failed build data
rm -rf ./redisearch

# 2. Run with the new tools directory at the front of the PATH

PATH="$HOME/falkor-build-tools:$PATH" \
CC="clang-20" \
CXX="clang++-20" \
./redisearch.sh
