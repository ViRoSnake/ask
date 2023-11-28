build:
	cargo build --release
	cp ./target/release/ask ~/bin/ask
	$(zsh source ~/.zshrc) 
