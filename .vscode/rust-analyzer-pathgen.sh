rm paths.txt

for dir in /home/user/learning/rust/*
do
    path="\"$dir/Cargo.toml\","
    echo "$path"
    echo "$path" >> paths.txt
done
