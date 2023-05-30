# Makefile for making the game of life program

CC = ./wasi-sdk-20.0/bin/clang
CFLAGS = -Wl,--export-all --sysroot wasi-sdk-20.0/share/wasi-sysroot/ -o hello.wasm -v


CFLAGS += $(shell pkg-config --cflags json-c)
LDFLAGS += $(shell pkg-config --libs json-c)


hello : hello.c
	$(CC) $(CFLAGS) hello.c $(LDFLAGS)

default: json

json: json_type.c
	$(CC) -O3 -o json json_type.c -lm

.PHONY: clean
clean:
	rm -Rf *.o lib/*.o json *.dSYM

.PHONY: package
package:
	tar -cvzf json.tar *