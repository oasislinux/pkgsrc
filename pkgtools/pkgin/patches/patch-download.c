$NetBSD$

--- download.c.orig	Sun May 24 23:15:44 2020
+++ download.c
@@ -30,7 +30,6 @@
 #include "pkgin.h"
 #include "external/progressmeter.h"
 
-int	fetchTimeout = 15; /* wait 15 seconds before timeout */
 size_t	fetch_buffer = 1024;
 
 /*
