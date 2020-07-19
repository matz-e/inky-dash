FROM rustembedded/cross:arm-unknown-linux-gnueabihf

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes \
        libdbus-1-dev:armhf libssl-dev:armhf \
        libc6-dev:armhf libsystemd-dev:armhf

ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
