all: hello

output.o: output.rs
	rustc --crate-type=lib output.rs

hello: hello.rs output.o
	rustc -L . hello.rs

clean:
	rm -rf *.o *.rlib hello