$NetBSD$

--- libcap/psx.c.orig	Mon Mar 30 03:10:29 2020
+++ libcap/psx.c
@@ -14,6 +14,7 @@
 
 #include <errno.h>
 #include <pthread.h>
+#include <sched.h>
 #include <signal.h>
 #include <stdarg.h>
 #include <stdio.h>
@@ -533,7 +534,7 @@ long int __psx_syscall(long int syscall_nr, ...) {
 	if (!waiting) {
 	    break;
 	}
-	pthread_yield();
+	sched_yield();
     }
 
     errno = restore_errno;
