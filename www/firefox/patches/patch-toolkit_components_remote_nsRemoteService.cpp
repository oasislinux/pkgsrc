$NetBSD$

--- toolkit/components/remote/nsRemoteService.cpp.orig	Mon Mar  9 13:10:20 2020
+++ toolkit/components/remote/nsRemoteService.cpp
@@ -157,6 +157,8 @@ void nsRemoteService::StartupServer() {
   if (!useX11Remote || getenv(DBUS_REMOTE_ENV)) {
     mRemoteServer = MakeUnique<nsDBusRemoteServer>();
   }
+#  else
+  return;
 #  endif
   if (!mRemoteServer && useX11Remote) {
     mRemoteServer = MakeUnique<nsGTKRemoteServer>();
