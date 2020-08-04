$NetBSD$

--- include/externals.h.orig	Tue Aug  4 08:41:05 2020
+++ include/externals.h
@@ -43,7 +43,7 @@
 #include <math.h>
 #include <assert.h>
 #include <fcntl.h>
-#include <sys/queue.h>
+#include <nbcompat/queue.h>
 #include <sys/stat.h>
 
 #include <xcb/xcb.h>
