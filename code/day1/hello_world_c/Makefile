CC ?= cc
CXX ?= c++

CFLAGS ?= -O2 -Wall -Wextra -pedantic -std=c11
CXXFLAGS ?= -O2 -Wall -Wextra -pedantic -std=c++17

.PHONY: all clean

all: hello_c hello_cpp

hello_c: hello.c
	$(CC) $(CFLAGS) -o $@ $<

hello_cpp: hello.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

clean:
	rm -f hello_c hello_cpp

