
APP_DIR= $(HOME)/.local/share/applications


all: binary desktop-entry

binary:
	cargo install --path .

desktop-entry:
	desktop-file-install --dir=$(APP_DIR) ./Pikcolior.desktop
	chmod u+x $(APP_DIR)/Pikcolior.desktop
