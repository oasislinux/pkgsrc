$NetBSD$

--- gdk/wayland/gdkdisplay-wayland.c.orig	Mon Jan 27 21:13:23 2020
+++ gdk/wayland/gdkdisplay-wayland.c
@@ -399,8 +399,8 @@ gdk_registry_handle_global (void               *data,
   if (strcmp (interface, "wl_compositor") == 0)
     {
       display_wayland->compositor =
-        wl_registry_bind (display_wayland->wl_registry, id, &wl_compositor_interface, MIN (version, 3));
-      display_wayland->compositor_version = MIN (version, 3);
+        wl_registry_bind (display_wayland->wl_registry, id, &wl_compositor_interface, MIN (version, 4));
+      display_wayland->compositor_version = MIN (version, 4);
     }
   else if (strcmp (interface, "wl_shm") == 0)
     {
