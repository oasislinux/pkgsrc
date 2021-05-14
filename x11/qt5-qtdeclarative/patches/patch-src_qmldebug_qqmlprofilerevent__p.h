$NetBSD$

Fix build with gcc 11.

--- src/qmldebug/qqmlprofilerevent_p.h.orig	Fri May 14 00:21:27 2021
+++ src/qmldebug/qqmlprofilerevent_p.h
@@ -48,6 +48,7 @@
 #include <QtCore/qmetatype.h>
 
 #include <initializer_list>
+#include <limits>
 #include <type_traits>
 
 //
