NAME = n-puzzle

all: $(NAME)

$(NAME): src/main.rs src/node/mod.rs src/heuristics/mod.rs Cargo.toml
	cargo build --release
	ln -sf target/release/$(NAME)

clean:

fclean:
	rm -f target/release/$(NAME)
	rm -f $(NAME)

re: fclean all
