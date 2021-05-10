BINS = target/release/git-prune-branches target/release/git-pick

.PHONY: install

install:
	@cargo build --release
	@strip $(BINS)
	@mkdir -p $(DESTDIR)/bin
	@cp $(BINS) $(DESTDIR)/bin
