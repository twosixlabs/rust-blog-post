all: main.cpp MyCppClass.cpp
	g++ -o MyCppApp main.cpp MyCppClass.cpp -I. -Wl,--unresolved-symbols=ignore-in-object-files -L../cppcwrapper/ -lcppcwrapper -pthread -ldl
