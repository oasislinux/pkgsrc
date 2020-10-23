$NetBSD: patch-Modules_Platform_UnixPaths.cmake,v 1.2 2020/07/31 09:02:31 wiz Exp $

Patch in pkgsrc paths for finding stuff.

--- Modules/Platform/UnixPaths.cmake.orig	2020-07-15 11:19:20.000000000 +0000
+++ Modules/Platform/UnixPaths.cmake
@@ -14,10 +14,12 @@ set(__UNIX_PATHS_INCLUDED 1)
 
 set(UNIX 1)
 
-# also add the install directory of the running cmake to the search directories
-# CMAKE_ROOT is CMAKE_INSTALL_PREFIX/share/cmake, so we need to go two levels up
-get_filename_component(_CMAKE_INSTALL_DIR "${CMAKE_ROOT}" PATH)
-get_filename_component(_CMAKE_INSTALL_DIR "${_CMAKE_INSTALL_DIR}" PATH)
+# If building under pkgsrc, we do not want @LOCALBASE@ or
+# CMAKE_INSTALL_PREFIX. Dependencies will be detected in $BUILDLINK_DIR,
+# which is added to CMAKE_SYSTEM_PREFIX_PATH on the command-line.
+if(NOT DEFINED ENV{BUILDLINK_DIR})
+  list(APPEND CMAKE_SYSTEM_PREFIX_PATH @LOCALBASE@)
+endif()
 
 # List common installation prefixes.  These will be used for all
 # search types.
@@ -28,15 +30,14 @@ get_filename_component(_CMAKE_INSTALL_DIR "${_CMAKE_IN
 list(APPEND CMAKE_SYSTEM_PREFIX_PATH
   # Standard
   /usr/local /usr /
-
-  # CMake install location
-  "${_CMAKE_INSTALL_DIR}"
   )
 if (NOT CMAKE_FIND_NO_INSTALL_PREFIX)
-  list(APPEND CMAKE_SYSTEM_PREFIX_PATH
-    # Project install destination.
-    "${CMAKE_INSTALL_PREFIX}"
-  )
+  if(NOT DEFINED ENV{BUILDLINK_DIR})
+    list(APPEND CMAKE_SYSTEM_PREFIX_PATH
+      # Project install destination.
+      "${CMAKE_INSTALL_PREFIX}"
+    )
+  endif()
   if(CMAKE_STAGING_PREFIX)
     list(APPEND CMAKE_SYSTEM_PREFIX_PATH
       # User-supplied staging prefix.
@@ -45,23 +46,13 @@ if (NOT CMAKE_FIND_NO_INSTALL_PREFIX)
   endif()
 endif()
 
-# Non "standard" but common install prefixes
-list(APPEND CMAKE_SYSTEM_PREFIX_PATH
-  /usr/X11R6
-  /usr/pkg
-  /opt
-  )
+# X11 install prefix
+if(NOT "@LOCALBASE@" STREQUAL "@X11BASE@")
+  list(APPEND CMAKE_SYSTEM_PREFIX_PATH @X11BASE@)
+endif()
 
-# List common include file locations not under the common prefixes.
-list(APPEND CMAKE_SYSTEM_INCLUDE_PATH
-  # X11
-  /usr/include/X11
-  )
-
-list(APPEND CMAKE_SYSTEM_LIBRARY_PATH
-  # X11
-  /usr/lib/X11
-  )
+# Non "standard" but common install prefix
+list(APPEND CMAKE_SYSTEM_PREFIX_PATH /opt)
 
 list(APPEND CMAKE_PLATFORM_IMPLICIT_LINK_DIRECTORIES
   /lib /lib32 /lib64 /usr/lib /usr/lib32 /usr/lib64
