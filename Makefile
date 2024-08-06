.PHONY: install doc

USER_APPS_DIR := $(HOME)/.local/share/applications


install: install-binary install-desktop

install-binary:
	cargo install --path .

install-desktop:
	desktop-file-install --dir=$(USER_APPS_DIR) ./Pikcolior.desktop
	chmod u+x $(USER_APPS_DIR)/Pikcolior.desktop

doc: ./target/debug/pikcolior
	mkdir -p ./target/doc/
	./target/debug/pikcolior --help > ./target/doc/help-output

./target/debug/pikcolior:
	cargo build
