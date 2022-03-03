all:
	cargo build
	ln -sf ./target/debug/builder-bees ./agent #TODO: Figure out how to output executable to root

clean:
	/bin/rm agent
