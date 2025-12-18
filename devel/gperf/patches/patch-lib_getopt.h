$NetBSD$

--- lib/getopt.h.orig	Thu Dec 18 20:44:11 2025
+++ lib/getopt.h
@@ -102,7 +102,7 @@ struct option
    errors, only prototype getopt for the GNU C library.  */
 extern int getopt (int argc, char *const *argv, const char *shortopts);
 #else /* not __GNU_LIBRARY__ */
-extern int getopt ();
+extern int getopt (int argc, char *const argv[], const char *optstring);
 #endif /* __GNU_LIBRARY__ */
 #endif /* __cplusplus */
 extern int getopt_long (int argc, char *const *argv, const char *shortopts,
