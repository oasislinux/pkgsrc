$NetBSD$

Fix build with gcc 11.

Upstream: https://code.qt.io/cgit/qt/qtbase.git/commit/?id=9c56d4da2ff631a8c1c30475bd792f6c86bda53c

--- src/corelib/global/qfloat16.h.orig	Thu May 13 20:29:33 2021
+++ src/corelib/global/qfloat16.h
@@ -43,6 +43,7 @@
 
 #include <QtCore/qglobal.h>
 #include <QtCore/qmetatype.h>
+#include <limits>
 #include <string.h>
 
 #if defined(QT_COMPILER_SUPPORTS_F16C) && defined(__AVX2__) && !defined(__F16C__)
