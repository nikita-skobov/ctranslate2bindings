FROM ubuntu:20.04


ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y wget python3-pip unzip cmake build-essential pkg-config libgoogle-perftools-dev git

RUN apt-get install -y libopenblas-dev

RUN apt-get install -y libomp-dev

WORKDIR /opt/

RUN git clone https://github.com/OpenNMT/CTranslate2

WORKDIR /opt/CTranslate2

RUN git submodule update --init --recursive

RUN mkdir build

WORKDIR /opt/CTranslate2/build

ARG SHARED=OFF

RUN cmake -DWITH_OPENBLAS=ON -DWITH_CUDA=OFF -DWITH_MKL=OFF -DBUILD_SHARED_LIBS=$SHARED -DOPENMP_RUNTIME=NONE .. && make -j4


RUN mv libctranslate2.a libctranslateold.a && \
    echo "create libctranslate2.a" >> libctranslate.mri && \
    echo "addlib libctranslateold.a" >> libctranslate.mri && \
    echo "addlib /lib/x86_64-linux-gnu/libopenblas.a" >> libctranslate.mri && \
    echo "addlib ./third_party/cpu_features/libcpu_features.a" >> libctranslate.mri && \
    echo "save" >> libctranslate.mri && \
    echo "end" >> libctranslate.mri

# combining all of the necessary .a libraries into one big static library
RUN ar -M <libctranslate.mri

COPY wrap.c /opt/

RUN cp /opt/wrap.c ./

RUN g++ -I../include/ -c wrap.c libctranslate2.a -lgomp -ldl -lm -pthread -o wrap.o

RUN ar crus libwrap.a wrap.o

RUN mv libctranslate2.a libctranslateold.a && \
    echo "create libctranslate2.a" > libctranslate.mri && \
    echo "addlib libctranslateold.a" >> libctranslate.mri && \
    echo "addlib libwrap.a" >> libctranslate.mri && \
    echo "save" >> libctranslate.mri && \
    echo "end" >> libctranslate.mri

RUN ar -M <libctranslate.mri
