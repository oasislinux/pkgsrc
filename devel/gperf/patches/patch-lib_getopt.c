$NetBSD$

--- lib/getopt.c.orig	Thu Dec 18 20:44:47 2025
+++ lib/getopt.c
@@ -193,7 +193,7 @@ static char *posixly_correct;
 /* Avoid depending on library functions or files
    whose names are inconsistent.  */
 
-extern char *getenv ();
+extern char *getenv (const char *name);
 
 static char *
 my_index (const char *str, int chr)
