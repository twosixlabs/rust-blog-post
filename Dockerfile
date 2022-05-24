FROM ubuntu:18.04

RUN apt-get -y update && apt-get install -y build-essential g++ curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal -y
ENV PATH /root/.cargo/bin:$PATH

WORKDIR /code/

CMD make && LD_LIBRARY_PATH=cppcwrapper/ ./cpp/MyCppApp
