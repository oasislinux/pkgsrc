$NetBSD$

--- linux-user/syscall.c.orig	Mon May 25 08:21:55 2020
+++ linux-user/syscall.c
@@ -123,6 +123,14 @@
 #include "fd-trans.h"
 #include "tcg/tcg.h"
 
+#ifndef F_SHLCK
+#define F_SHLCK                 8
+#endif
+
+#ifndef F_EXLCK
+#define F_EXLCK                 4
+#endif
+
 #ifndef CLONE_IO
 #define CLONE_IO                0x80000000      /* Clone io context */
 #endif
@@ -6771,9 +6779,20 @@ static inline abi_long host_to_target_timex(abi_long t
 }
 #endif
 
-static inline abi_long target_to_host_sigevent(struct sigevent *host_sevp,
+struct host_sigevent {
+    union sigval sigev_value;
+    int sigev_signo;
+    int sigev_notify;
+    union {
+       int _pad[64-sizeof(int) * 2 + sizeof(union sigval)];
+       int _tid;
+    } _sigev_un;
+};
+
+static inline abi_long target_to_host_sigevent(struct sigevent *sevp,
                                                abi_ulong target_addr)
 {
+    struct host_sigevent *host_sevp = (struct host_sigevent *)sevp;
     struct target_sigevent *target_sevp;
 
     if (!lock_user_struct(VERIFY_READ, target_sevp, target_addr, 1)) {
