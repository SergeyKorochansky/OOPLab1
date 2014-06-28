all: hello

input.o: input.rs
	rustc --crate-type=lib input.rs

hello: hello.rs input.o
	rustc -L . hello.rs

clean:
	rm -rf *.o *.rlib hello