$NetBSD$

musl does not have pthread_getname_np and mapbox-gl is only built
on Linux, so only use pthread_getname_np with glibc.

--- src/3rdparty/mapbox-gl-native/platform/default/thread.cpp.orig	Fri Aug 14 04:22:05 2020
+++ src/3rdparty/mapbox-gl-native/platform/default/thread.cpp
@@ -11,7 +11,9 @@ namespace platform {
 
 std::string getCurrentThreadName() {
     char name[32] = "unknown";
+#ifdef __GLIBC__
     pthread_getname_np(pthread_self(), name, sizeof(name));
+#endif
 
     return name;
 }
