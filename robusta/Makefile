java_run: lib
	javac Main.java && RUST_BACKTRACE=full java -Djava.library.path=mylib/target/debug/ Main

.PHONY: lib

rust:
	cd mylib && cargo build
