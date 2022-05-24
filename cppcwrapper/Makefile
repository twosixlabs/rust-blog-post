all: RustStructWrapper.cpp
	g++ -fPIC -shared RustStructWrapper.cpp -o libcppcwrapper.so -L../rustcwrapper/target/debug/ -lrustcwrapper
