all:
	cargo build
	ln -sf ./target/debug/builder-bees ./agent #TODO: Figure out how to output executable to root

#TODO: Create separate build options for the two agents
simple_agent:
	cargo build
	ln -sf ./target/debug/builder-bees ./agent

clean:
	/bin/rm agent
