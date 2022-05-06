language=${1}

for file in src/"$language"/*.md ; do
    echo Checking references in $file
    cargo run --quiet --bin link2print < $file > /dev/null
done
