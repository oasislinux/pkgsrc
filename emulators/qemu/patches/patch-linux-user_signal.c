$NetBSD$

--- linux-user/signal.c.orig	Fri Feb 28 20:08:38 2020
+++ linux-user/signal.c
@@ -25,6 +25,14 @@
 #include "trace.h"
 #include "signal-common.h"
 
+#ifndef __SIGRTMIN
+#define __SIGRTMIN 32
+#endif
+
+#ifndef __SIGRTMAX
+#define __SIGRTMAX (NSIG-1)
+#endif
+
 static struct target_sigaction sigact_table[TARGET_NSIG];
 
 static void host_signal_handler(int host_signum, siginfo_t *info,
