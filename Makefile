run: main
	./main

main: src/main.c
	gcc src/main.c -lraylib -lGL -lm -lpthread -ldl -lrt -lX11 -o main

clean:
	rm main
