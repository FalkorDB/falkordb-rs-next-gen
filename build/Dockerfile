FROM redis

ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt update -y && \
    apt install -y software-properties-common wget && \
    wget https://apt.llvm.org/llvm.sh && \
    chmod +x llvm.sh && \
    ./llvm.sh 20 && \
    apt update -y

RUN apt install -y git make cmake curl zlib1g zlib1g-dev libpolly-20-dev libomp-20-dev libzstd-dev python3-venv lcov

RUN python3 -m venv venv

RUN venv/bin/pip install redis pytest

RUN git clone https://github.com/DrTimothyAldenDavis/GraphBLAS.git

RUN cd GraphBLAS && \
    make static CMAKE_OPTIONS='-DGRAPHBLAS_COMPACT=1 -DCMAKE_POSITION_INDEPENDENT_CODE=on' JOBS=2 CC=clang-20 CXX=clang++-20 && \
    make install && \
    cd .. && \
    rm -rf GraphBLAS

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

CMD "/bin/bash"