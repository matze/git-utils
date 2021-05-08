BIN = target/release/git-prune-branches

.PHONY: install

install:
	@cargo build --release
	@strip $(BIN)
	@mkdir -p $(DESTDIR)/bin
	@cp $(BIN) $(DESTDIR)/bin
