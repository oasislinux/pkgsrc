$NetBSD$

Fix build with gcc 11.

--- src/3rdparty/masm/yarr/Yarr.h.orig	Thu May 13 22:11:49 2021
+++ src/3rdparty/masm/yarr/Yarr.h
@@ -27,6 +27,7 @@
 
 #pragma once
 
+#include <limits>
 #include <limits.h>
 #include "YarrErrorCode.h"
 
