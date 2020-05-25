$NetBSD$

Fix build with gcc 10, which enables -fno-common by default. Upstream
pull request at https://github.com/hishamhm/htop/pull/981.

--- CRT.h.orig	Mon May 25 04:53:53 2020
+++ CRT.h
@@ -140,7 +140,7 @@ extern const char **CRT_treeStr;
 
 extern int CRT_delay;
 
-int* CRT_colors;
+extern int* CRT_colors;
 
 extern int CRT_colorSchemes[LAST_COLORSCHEME][LAST_COLORELEMENT];
 
@@ -150,13 +150,13 @@ extern int CRT_scrollHAmount;
 
 extern int CRT_scrollWheelVAmount;
 
-char* CRT_termType;
+extern char* CRT_termType;
 
 // TODO move color scheme to Settings, perhaps?
 
 extern int CRT_colorScheme;
 
-void *backtraceArray[128];
+extern void *backtraceArray[128];
 
 #if HAVE_SETUID_ENABLED
 
