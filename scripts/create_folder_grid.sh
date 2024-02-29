#!/bin/bash

create_file_test()
{
    mkdir "$2"test_"$1"
    for i in $(seq 1 5)
    do
        python2 ../scripts/npuzzle-gen.py $1 -$2 > "$2"test_"$1"/"test_$i".txt
    done
}

create_all_test_3()
{
    create_file_test "3" "s" 
    create_file_test "3" "u" 
}

create_test_4()
{
    mkdir stest_4
    mkdir utest_4
    python2 ../scripts/npuzzle-gen.py 4 -s > stest_4/test_1.txt
    python2 ../scripts/npuzzle-gen.py 4 -u > utest_4/test_1.txt
}

cd target
create_all_test_3
create_test_4