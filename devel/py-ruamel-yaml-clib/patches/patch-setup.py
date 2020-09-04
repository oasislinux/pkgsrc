$NetBSD: patch-setup.py,v 1.1 2020/09/02 21:42:48 wiz Exp $

Compatibility with setuptools>=50.0.0
https://github.com/pypa/setuptools/issues/2355#issuecomment-685159580

--- setup.py.orig	2019-09-27 06:04:08.000000000 +0000
+++ setup.py
@@ -447,10 +447,7 @@ class NameSpacePackager(object):
             sys.exit(1)
 
     def check(self):
-        try:
-            from pip.exceptions import InstallationError
-        except ImportError:
-            return
+        InstallationError = Exception
         # arg is either develop (pip install -e) or install
         if self.command not in ['install', 'develop']:
             return
