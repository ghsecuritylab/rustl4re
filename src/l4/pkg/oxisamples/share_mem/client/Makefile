PKGDIR	= .
L4DIR	?= $(PKGDIR)/../../../..

TARGET		= shm_rust-client
SRC_RS		= client.rs

REQUIRES_LIBS    = libdl libpthread libc_be_sig l4re_c-util \
		libc_support_misc l4-sys-rust l4re-rust l4_derive-rust l4-rust libl4re-wrapper \
		l4re_c

include $(L4DIR)/mk/prog.mk
