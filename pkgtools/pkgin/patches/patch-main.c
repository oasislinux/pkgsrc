$NetBSD$

--- main.c.orig	Sun May 24 23:15:47 2020
+++ main.c
@@ -55,6 +55,9 @@ main(int argc, char *argv[])
 	/* Default to not doing \r printouts if we don't send to a tty */
 	parsable = !isatty(fileno(stdout));
 
+	/* Wait 15 seconds before timeout */
+	fetchTimeout = 15;
+
 	while ((ch = getopt(argc, argv, "dhyfFPvVl:nc:t:p")) != -1) {
 		switch (ch) {
 		case 'f':
