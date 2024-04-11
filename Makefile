run: main
	./main
	rm main

main:
	clang \
		$(shell pkg-config --cflags gtk4) \
		$(shell pkg-config --cflags --libs libmongoc-1.0) \
		-o main src/main.c $(shell pkg-config --libs gtk4)

