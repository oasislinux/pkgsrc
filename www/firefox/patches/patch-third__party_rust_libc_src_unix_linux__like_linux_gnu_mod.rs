$NetBSD$

Fix build with musl libc.

Upstream: https://github.com/rust-lang/libc/pull/2122

--- third_party/rust/libc/src/unix/linux_like/linux/gnu/mod.rs.orig	Tue May  4 19:00:06 2021
+++ third_party/rust/libc/src/unix/linux_like/linux/gnu/mod.rs
@@ -1055,33 +1055,6 @@ pub const STATX_ATTR_NODUMP: ::c_int = 0x0040;
 pub const STATX_ATTR_ENCRYPTED: ::c_int = 0x0800;
 pub const STATX_ATTR_AUTOMOUNT: ::c_int = 0x1000;
 
-// sys/auxv.h
-pub const AT_NULL: ::c_ulong = 0;
-pub const AT_IGNORE: ::c_ulong = 1;
-pub const AT_EXECFD: ::c_ulong = 2;
-pub const AT_PHDR: ::c_ulong = 3;
-pub const AT_PHENT: ::c_ulong = 4;
-pub const AT_PHNUM: ::c_ulong = 5;
-pub const AT_PAGESZ: ::c_ulong = 6;
-pub const AT_BASE: ::c_ulong = 7;
-pub const AT_FLAGS: ::c_ulong = 8;
-pub const AT_ENTRY: ::c_ulong = 9;
-pub const AT_NOTELF: ::c_ulong = 10;
-pub const AT_UID: ::c_ulong = 11;
-pub const AT_EUID: ::c_ulong = 12;
-pub const AT_GID: ::c_ulong = 13;
-pub const AT_EGID: ::c_ulong = 14;
-pub const AT_PLATFORM: ::c_ulong = 15;
-pub const AT_HWCAP: ::c_ulong = 16;
-pub const AT_CLKTCK: ::c_ulong = 17;
-// AT_* values 18 through 22 are reserved
-pub const AT_SECURE: ::c_ulong = 23;
-pub const AT_BASE_PLATFORM: ::c_ulong = 24;
-pub const AT_RANDOM: ::c_ulong = 25;
-pub const AT_HWCAP2: ::c_ulong = 26;
-
-pub const AT_EXECFN: ::c_ulong = 31;
-
 //sys/timex.h
 pub const ADJ_OFFSET: ::c_uint = 0x0001;
 pub const ADJ_FREQUENCY: ::c_uint = 0x0002;
