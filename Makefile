all:
	cargo build
	ln -sf ./debug/builder-bees ./agent #TODO: Figure out how to output executable to root

clean:
	/bin/rm agent
