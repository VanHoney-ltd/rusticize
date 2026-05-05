PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
SHAREDIR = $(PREFIX)/share

.PHONY: all install uninstall

all:
	cargo build --release

install:
	install -Dm755 target/release/rusticize $(DESTDIR)$(BINDIR)/rusticize
	install -Dm755 packaging/rusticize-launcher $(DESTDIR)$(BINDIR)/rusticize-launcher
	install -Dm644 packaging/rusticize.desktop $(DESTDIR)$(SHAREDIR)/applications/rusticize.desktop
	install -Dm644 packaging/rusticize.png $(DESTDIR)$(SHAREDIR)/pixmaps/rusticize.png

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/rusticize
	rm -f $(DESTDIR)$(BINDIR)/rusticize-launcher
	rm -f $(DESTDIR)$(SHAREDIR)/applications/rusticize.desktop
	rm -f $(DESTDIR)$(SHAREDIR)/pixmaps/rusticize.png
