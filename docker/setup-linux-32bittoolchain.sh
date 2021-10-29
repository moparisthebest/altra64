#!/bin/bash
#
# Copyright (c) 2017 The Altra64 project contributors
# See LICENSE file in the project root for full license information.
#

set -euxo pipefail

# Download and install latest updates for the system [sudo req.]
apt-get update
apt-get -y upgrade

# Install essential packages [sudo req.]
apt-get -y install wget build-essential git texinfo libc6 libgmp-dev libmpfr-dev libmpc-dev libpng-dev zlib1g-dev libtool autoconf

# change to the users root directory
cd ~/

# add a system variable and make it perminent
# echo 'N64_INST=/usr/local/libdragon' >> /etc/environment
# echo 'export N64_INST=/usr/local/libdragon' >> ~/.bashrc
export N64_INST=/usr/local/libdragon

# EDIT THIS LINE TO CHANGE YOUR INSTALL PATH!
export INSTALL_PATH=/usr/local/libdragon

# Set up path for newlib to compile later
export PATH=$PATH:$INSTALL_PATH/bin

# Versions
export BINUTILS_V=2.27
export GCC_V=6.2.0
export NEWLIB_V=2.4.0


export BINUTILS_V=2.37
#export GCC_V=11.2.0
#export NEWLIB_V=4.1.0


export GCC_V=4.6.4

# make a build folder for libdragon
mkdir build_gcc
cd build_gcc

# Download stage
#wget -c ftp://sourceware.org/pub/binutils/releases/binutils-$BINUTILS_V.tar.bz2
wget -c https://sourceware.org/pub/binutils/releases/binutils-$BINUTILS_V.tar.xz
wget -c https://sourceware.org/pub/gcc/releases/gcc-$GCC_V/gcc-$GCC_V.tar.bz2
#wget -c https://sourceware.org/pub/gcc/releases/gcc-$GCC_V/gcc-$GCC_V.tar.xz
wget -c https://sourceware.org/pub/newlib/newlib-$NEWLIB_V.tar.gz

# Extract stage
test -d binutils-$BINUTILS_V || tar -xvJf binutils-$BINUTILS_V.tar.xz || tar -xvjf binutils-$BINUTILS_V.tar.bz2
test -d gcc-$GCC_V || tar -xvJf gcc-$GCC_V.tar.xz || tar -xvjf gcc-$GCC_V.tar.bz2
test -d newlib-$NEWLIB_V || tar -xvzf newlib-$NEWLIB_V.tar.gz

# Binutils and newlib support compiling in source directory, GCC does not

# Compile binutils
cd binutils-$BINUTILS_V
./configure --prefix=${INSTALL_PATH} --target=mips64-elf --with-cpu=mips64vr4300 --disable-werror
make -j9
make install
cd ..

# Compile gcc (pass 1)
rm -rf gcc_compile
mkdir gcc_compile
cd gcc_compile
CFLAGS_FOR_TARGET="-G0 -mabi=32 -march=vr4300 -mtune=vr4300 -O2" ../gcc-$GCC_V/configure --prefix=${INSTALL_PATH} --target=mips64-elf --enable-languages=c --without-headers --with-newlib --with-system-zlib --disable-libssp --enable-multilib --disable-shared --with-gcc --with-gnu-ld --with-gnu-as --disable-threads --disable-win32-registry --disable-nls --disable-debug --disable-libmudflap --disable-werror
make -j9
make install
cd ..

# hacky hack hack
mv /usr/local/libdragon/bin/mips64-elf-gcc /usr/local/libdragon/bin/mips64-elf-gcc.orig
cat > /usr/local/libdragon/bin/mips64-elf-gcc <<EOF
#!/bin/bash
set +x
exec /usr/local/libdragon/bin/mips64-elf-gcc.orig -mabi=32 -mtune=vr4300 -march=vr4300 "\$@"
EOF
chmod +x /usr/local/libdragon/bin/mips64-elf-gcc

# Compile newlib
cd newlib-$NEWLIB_V
CFLAGS_FOR_TARGET="-G0 -march=vr4300 -mtune=vr4300 -O2" CFLAGS="-O2" CXXFLAGS="-O2" ./configure --target=mips64-elf --prefix=${INSTALL_PATH} --with-cpu=mips64vr4300 --disable-threads --disable-libssp  --disable-werror
make -j9
make install
cd ..

# Compile gcc (pass 2)
#rm -rf gcc_compile
#mkdir gcc_compile
#cd gcc_compile
#CFLAGS_FOR_TARGET="-G0 -mabi=32 -march=vr4300 -mtune=vr4300 -O2" CXXFLAGS_FOR_TARGET="-G0 -mabi=32 -march=vr4300 -mtune=vr4300 -O2" ../gcc-#$GCC_V/configure --prefix=${INSTALL_PATH} --target=mips64-elf --enable-languages=c,c++ --with-newlib --with-system-zlib --disable-libssp --#enable-multilib --disable-shared --with-gcc --with-gnu-ld --with-gnu-as --disable-threads --disable-win32-registry --disable-nls --disable-#debug --disable-libmudflap
#make -j9
#make install
#cd ..

export CFLAGS="-std=gnu99 -mabi=32 -march=vr4300 -mtune=vr4300"

export CFLAGS="-std=gnu99 -march=vr4300 -mtune=vr4300"

# Pull the latest libdragon source code and make a build directory
git clone https://github.com/dragonminded/libdragon.git
# set to correct commit
cd libdragon
git checkout -f b26fce6

# fix issues with the build scripts
sed -i -- 's|${N64_INST:-/usr/local}|/usr/local/libdragon|g' tools/build
sed -i -- 's|--with-newlib|--with-newlib --with-system-zlib|g' tools/build

sed -i -- 's| -lpng|\nLDLIBS = -lpng|g' tools/mksprite/Makefile
sed -i -- 's| -Werror| -w|g' tools/mksprite/Makefile
# run the install script
#find -type f -name Makefile -print0 | xargs -0 sed -i 's/ -mtune=vr4300 / -mabi=32 -mtune=vr4300 /'
make -j9
make install
make -j9 tools
make tools-install

export LDFLAGS="-L$N64_INST/lib -Tn64ld.x"
export LIBS="-ldragon -lc -ldragonsys -lnosys"

cd ..
# install libmikmod (custom version)
git clone https://github.com/n64-tools/libmikmod
cd libmikmod/n64
make -j9
make install
cd .. # we have are in a subfolder, this is not a duplicate...

cd ..
# install libyaml
git clone https://github.com/yaml/libyaml
cd libyaml
./bootstrap
./configure --host=mips64-elf --prefix=$N64_INST
make -j9
make install

cd ..
# install libmad (custom version)
git clone https://github.com/n64-tools/libmad
cd libmad
./configure --host=mips64-elf --prefix=$N64_INST
# needed for GCC 4.6.4
sed -i 's/-fforce-mem//' Makefile
make -j9
make install

cd ..

# Perform cleanup
apt-get -y autoremove
apt-get autoclean

find /usr/local/libdragon/bin /usr/local/libdragon/mips64-elf/bin /usr/local/libdragon/libexec/gcc/mips64-elf -type f -print0 | xargs -0 strip || true

# these are ENV in Dockerfile now
#echo 'export N64_INST=/usr/local/libdragon' >> ~/.bashrc
#echo 'export PATH="$PATH:$N64_INST/bin"' >> ~/.bashrc
