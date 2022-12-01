java_run: lib
	javac Main.java && java -Djava.library.path=mylib/target/debug/ Main

.PHONY: lib

javah:
	javac -h . Main.java

rust:
	cd mylib && cargo build
