
USER_APPS_DIR := $(HOME)/.local/share/applications


install: install-binary install-desktop

install-binary:
	cargo install --path .

install-desktop:
	desktop-file-install --dir=$(USER_APPS_DIR) ./Pikcolior.desktop
	chmod u+x $(USER_APPS_DIR)/Pikcolior.desktop
