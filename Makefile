all: hello

hello: hello.rs
	rustc hello.rs -o hello

clean:
	rm -rf *.o hello