# C++ Courier Client

Initialize submodules

    > git submodule update --init --recursive

Initialize cmake

    > mkdir build
    > cd build
    > cmake .. -DCMAKE_INSTALL_PREFIX=../install
    > cd ..

Build the courier client

    > cmake --build build -- -j

Run the courier client tests

    > ./build/courier_test

Install the courier client

    > cd build
    > make install
