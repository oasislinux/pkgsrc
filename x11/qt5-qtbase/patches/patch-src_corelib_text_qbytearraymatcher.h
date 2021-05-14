$NetBSD$

Fix build with gcc 11.

Upstream: https://code.qt.io/cgit/qt/qtbase.git/commit/?id=813a928c7c3cf98670b6043149880ed5c955efb9

--- src/corelib/text/qbytearraymatcher.h.orig	Thu May 13 20:58:58 2021
+++ src/corelib/text/qbytearraymatcher.h
@@ -42,6 +42,8 @@
 
 #include <QtCore/qbytearray.h>
 
+#include <limits>
+
 QT_BEGIN_NAMESPACE
 
 
