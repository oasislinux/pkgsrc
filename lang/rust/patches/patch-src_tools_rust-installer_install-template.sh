$NetBSD: patch-src_tools_rust-installer_install-template.sh,v 1.3 2018/11/27 15:45:23 adam Exp $

No logging to 'install.log'.
Do not create 'uninstall.sh'.
Remove non-standard tr(1) escapes (https://github.com/rust-lang/rust-installer/pull/99).

--- src/tools/rust-installer/install-template.sh.orig	Mon Dec 16 15:38:31 2019
+++ src/tools/rust-installer/install-template.sh
@@ -15,20 +15,12 @@ set -u
 init_logging() {
     local _abs_libdir="$1"
     local _logfile="$_abs_libdir/$TEMPLATE_REL_MANIFEST_DIR/install.log"
-    rm -f "$_logfile"
-    need_ok "failed to remove old installation log"
-    touch "$_logfile"
-    need_ok "failed to create installation log"
     LOGFILE="$_logfile"
 }
 
 log_line() {
     local _line="$1"
 
-    if [ -n "${LOGFILE-}" -a -e "${LOGFILE-}" ]; then
-	echo "$_line" >> "$LOGFILE"
-	# Ignore errors, which may happen e.g. after the manifest dir is deleted
-    fi
 }
 
 msg() {
@@ -169,7 +161,7 @@ valopt() {
     local doc="$*"
     if [ $HELP -eq 0 ]
     then
-        local uop=$(echo $op | tr '[:lower:]' '[:upper:]' | tr '\-' '\_')
+        local uop=$(echo $op | tr 'a-z-' 'A-Z_')
         local v="CFG_${uop}"
         eval $v="$default"
         for arg in $CFG_ARGS
@@ -972,7 +964,6 @@ write_to_file "$TEMPLATE_RUST_INSTALLER_VERSION" "$abs
 critical_need_ok "failed to write installer version"
 
 # Install the uninstaller
-install_uninstaller "$src_dir" "$src_basename" "$abs_libdir"
 
 # Install each component
 install_components "$src_dir" "$abs_libdir" "$dest_prefix" "$components"
