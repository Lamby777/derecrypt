.PHONY: all build install uninstall

prefix ?= /usr/local
bindir ?= $(prefix)/bin
datadir ?= /usr/share

all: build

build:
	cargo build --release --locked

install:
	# Install the binary to the specified bin directory
	install -Dm755 target/release/derecrypt $(DESTDIR)$(bindir)/derecrypt
	
	# Install the desktop entry
	install -Dm644 meta/org.sparklet.derecrypt.desktop $(DESTDIR)$(datadir)/applications/org.sparklet.derecrypt.desktop
	
	# Install the icon
	install -Dm644 meta/icon.png $(DESTDIR)$(datadir)/icons/hicolor/48x48/apps/derecrypt.png

uninstall:
	rm -f $(DESTDIR)$(bindir)/derecrypt
	rm -f $(DESTDIR)$(datadir)/icons/hicolor/48x48/apps/derecrypt.png
	rm -f $(DESTDIR)$(datadir)/applications/org.sparklet.derecrypt.desktop
