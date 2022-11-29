$NetBSD: patch-Source_WebKit_PlatformGTK.cmake,v 1.3 2019/03/13 20:49:41 leot Exp $

SunOS ld does not support --version-script.

--- Source/WebKit/PlatformGTK.cmake.orig	Wed Sep 14 11:58:10 2022
+++ Source/WebKit/PlatformGTK.cmake
@@ -42,7 +42,7 @@ add_definitions(-DLOCALEDIR="${CMAKE_INSTALL_FULL_LOCA
 add_definitions(-DDATADIR="${CMAKE_INSTALL_FULL_DATADIR}")
 add_definitions(-DLIBDIR="${LIB_INSTALL_DIR}")
 
-if (NOT DEVELOPER_MODE AND NOT CMAKE_SYSTEM_NAME MATCHES "Darwin")
+if (NOT DEVELOPER_MODE AND NOT CMAKE_SYSTEM_NAME MATCHES "Darwin" AND NOT CMAKE_SYSTEM_NAME MATCHES "SunOS")
     WEBKIT_ADD_TARGET_PROPERTIES(WebKit LINK_FLAGS "-Wl,--version-script,${CMAKE_CURRENT_SOURCE_DIR}/webkitglib-symbols.map")
 endif ()
 
@@ -677,27 +677,27 @@ add_custom_target(WebKit-forwarding-headers
 add_custom_command(
     OUTPUT ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2
     DEPENDS ${WEBKIT_DIR}/UIProcess/API/gtk
-    COMMAND ln -n -s -f ${WEBKIT_DIR}/UIProcess/API/gtk ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2
+    COMMAND ln -s -f ${WEBKIT_DIR}/UIProcess/API/gtk ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2
 )
 add_custom_command(
     OUTPUT ${WebKit2Gtk_DERIVED_SOURCES_DIR}/webkit2
     DEPENDS ${WebKit2Gtk_DERIVED_SOURCES_DIR}/webkit
-    COMMAND ln -n -s -f ${WebKit2Gtk_DERIVED_SOURCES_DIR}/webkit ${WebKit2Gtk_DERIVED_SOURCES_DIR}/webkit2
+    COMMAND ln -s -f ${WebKit2Gtk_DERIVED_SOURCES_DIR}/webkit ${WebKit2Gtk_DERIVED_SOURCES_DIR}/webkit2
 )
 add_custom_command(
     OUTPUT ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-${WEBKITGTK_API_VERSION}/webkit2
     DEPENDS ${WEBKIT_DIR}/UIProcess/API/gtk${GTK_API_VERSION}
-    COMMAND ln -n -s -f ${WEBKIT_DIR}/UIProcess/API/gtk${GTK_API_VERSION} ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-${WEBKITGTK_API_VERSION}/webkit2
+    COMMAND ln -s -f ${WEBKIT_DIR}/UIProcess/API/gtk${GTK_API_VERSION} ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-${WEBKITGTK_API_VERSION}/webkit2
 )
 add_custom_command(
     OUTPUT ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-webextension/webkit2
     DEPENDS ${WEBKIT_DIR}/WebProcess/InjectedBundle/API/gtk
-    COMMAND ln -n -s -f ${WEBKIT_DIR}/WebProcess/InjectedBundle/API/gtk ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-webextension/webkit2
+    COMMAND ln -s -f ${WEBKIT_DIR}/WebProcess/InjectedBundle/API/gtk ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-webextension/webkit2
 )
 add_custom_command(
     OUTPUT ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-webextension/webkitdom
     DEPENDS ${WEBKIT_DIR}/WebProcess/InjectedBundle/API/gtk/DOM
-    COMMAND ln -n -s -f ${WEBKIT_DIR}/WebProcess/InjectedBundle/API/gtk/DOM ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-webextension/webkitdom
+    COMMAND ln -s -f ${WEBKIT_DIR}/WebProcess/InjectedBundle/API/gtk/DOM ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2gtk-webextension/webkitdom
 )
 add_custom_target(WebKit-fake-api-headers
     DEPENDS ${WebKit2Gtk_FRAMEWORK_HEADERS_DIR}/webkit2
