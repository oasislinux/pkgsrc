$NetBSD$

Fix build with musl libc.

Upstream: https://github.com/rust-lang/libc/pull/2122

--- third_party/rust/libc/src/unix/linux_like/linux/mod.rs.orig	Tue May  4 18:59:46 2021
+++ third_party/rust/libc/src/unix/linux_like/linux/mod.rs
@@ -1086,6 +1086,32 @@ pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_int = 24
 pub const RLIM_SAVED_MAX: ::rlim_t = RLIM_INFINITY;
 pub const RLIM_SAVED_CUR: ::rlim_t = RLIM_INFINITY;
 
+pub const AT_NULL: ::c_ulong = 0;
+pub const AT_IGNORE: ::c_ulong = 1;
+pub const AT_EXECFD: ::c_ulong = 2;
+pub const AT_PHDR: ::c_ulong = 3;
+pub const AT_PHENT: ::c_ulong = 4;
+pub const AT_PHNUM: ::c_ulong = 5;
+pub const AT_PAGESZ: ::c_ulong = 6;
+pub const AT_BASE: ::c_ulong = 7;
+pub const AT_FLAGS: ::c_ulong = 8;
+pub const AT_ENTRY: ::c_ulong = 9;
+pub const AT_NOTELF: ::c_ulong = 10;
+pub const AT_UID: ::c_ulong = 11;
+pub const AT_EUID: ::c_ulong = 12;
+pub const AT_GID: ::c_ulong = 13;
+pub const AT_EGID: ::c_ulong = 14;
+pub const AT_PLATFORM: ::c_ulong = 15;
+pub const AT_HWCAP: ::c_ulong = 16;
+pub const AT_CLKTCK: ::c_ulong = 17;
+
+pub const AT_SECURE: ::c_ulong = 23;
+pub const AT_BASE_PLATFORM: ::c_ulong = 24;
+pub const AT_RANDOM: ::c_ulong = 25;
+pub const AT_HWCAP2: ::c_ulong = 26;
+
+pub const AT_EXECFN: ::c_ulong = 31;
+
 pub const GLOB_ERR: ::c_int = 1 << 0;
 pub const GLOB_MARK: ::c_int = 1 << 1;
 pub const GLOB_NOSORT: ::c_int = 1 << 2;
