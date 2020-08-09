$NetBSD$

--- media/audioipc/client/src/context.rs.orig	Mon Jul 20 22:49:41 2020
+++ media/audioipc/client/src/context.rs
@@ -14,10 +14,13 @@ use audioipc::codec::LengthDelimitedCodec;
 use audioipc::frame::{framed, Framed};
 use audioipc::platformhandle_passing::{framed_with_platformhandles, FramedWithPlatformHandles};
 use audioipc::{core, rpc};
-use audioipc::{messages, messages::DeviceCollectionReq, messages::DeviceCollectionResp, ClientMessage, ServerMessage};
+use audioipc::{
+    messages, messages::DeviceCollectionReq, messages::DeviceCollectionResp, ClientMessage,
+    ServerMessage,
+};
 use cubeb_backend::{
-    ffi, Context, ContextOps, DeviceCollectionRef, DeviceId, DeviceType, Error, Ops, Result, Stream, StreamParams,
-    StreamParamsRef,
+    ffi, Context, ContextOps, DeviceCollectionRef, DeviceId, DeviceType, Error, Ops, Result,
+    Stream, StreamParams, StreamParamsRef,
 };
 use futures::Future;
 use futures_cpupool::{CpuFuture, CpuPool};
@@ -35,8 +38,10 @@ struct CubebClient;
 impl rpc::Client for CubebClient {
     type Request = ServerMessage;
     type Response = ClientMessage;
-    type Transport =
-        FramedWithPlatformHandles<audioipc::AsyncMessageStream, LengthDelimitedCodec<Self::Request, Self::Response>>;
+    type Transport = FramedWithPlatformHandles<
+        audioipc::AsyncMessageStream,
+        LengthDelimitedCodec<Self::Request, Self::Response>,
+    >;
 }
 
 pub const CLIENT_OPS: Ops = capi_new!(ClientContext, ClientStream);
@@ -136,12 +141,16 @@ impl rpc::Server for DeviceCollectionServer {
     type Request = DeviceCollectionReq;
     type Response = DeviceCollectionResp;
     type Future = CpuFuture<Self::Response, ()>;
-    type Transport = Framed<audioipc::AsyncMessageStream, LengthDelimitedCodec<Self::Response, Self::Request>>;
+    type Transport =
+        Framed<audioipc::AsyncMessageStream, LengthDelimitedCodec<Self::Response, Self::Request>>;
 
     fn process(&mut self, req: Self::Request) -> Self::Future {
         match req {
             DeviceCollectionReq::DeviceChange(device_type) => {
-                trace!("ctx_thread: DeviceChange Callback: device_type={}", device_type);
+                trace!(
+                    "ctx_thread: DeviceChange Callback: device_type={}",
+                    device_type
+                );
 
                 let devtype = cubeb_backend::DeviceType::from_bits_truncate(device_type);
 
@@ -157,10 +166,14 @@ impl rpc::Server for DeviceCollectionServer {
                 self.cpu_pool.spawn_fn(move || {
                     run_in_callback(|| {
                         if devtype.contains(cubeb_backend::DeviceType::INPUT) {
-                            unsafe { input_cb.unwrap()(ptr::null_mut(), input_user_ptr as *mut c_void) }
+                            unsafe {
+                                input_cb.unwrap()(ptr::null_mut(), input_user_ptr as *mut c_void)
+                            }
                         }
                         if devtype.contains(cubeb_backend::DeviceType::OUTPUT) {
-                            unsafe { output_cb.unwrap()(ptr::null_mut(), output_user_ptr as *mut c_void) }
+                            unsafe {
+                                output_cb.unwrap()(ptr::null_mut(), output_user_ptr as *mut c_void)
+                            }
                         }
                     });
 
@@ -193,7 +206,8 @@ impl ContextOps for ClientContext {
         let thread_create_callback = params.thread_create_callback;
         let thread_destroy_callback = params.thread_destroy_callback;
 
-        let server_stream = unsafe { audioipc::MessageStream::from_raw_fd(params.server_connection) };
+        let server_stream =
+            unsafe { audioipc::MessageStream::from_raw_fd(params.server_connection) };
 
         let core = core::spawn_thread(
             "AudioIPC Client RPC",
@@ -217,8 +231,8 @@ impl ContextOps for ClientContext {
         // will return errors the caller expects to handle.
         let _ = send_recv!(rpc, ClientConnect(std::process::id()) => ClientConnected);
 
-        let backend_id =
-            send_recv!(rpc, ContextGetBackendId => ContextBackendId()).unwrap_or_else(|_| "(remote error)".to_string());
+        let backend_id = send_recv!(rpc, ContextGetBackendId => ContextBackendId())
+            .unwrap_or_else(|_| "(remote error)".to_string());
         let backend_id = CString::new(backend_id).expect("backend_id query failed");
 
         let cpu_pool = futures_cpupool::Builder::new()
@@ -263,7 +277,11 @@ impl ContextOps for ClientContext {
         send_recv!(self.rpc(), ContextGetPreferredSampleRate => ContextPreferredSampleRate())
     }
 
-    fn enumerate_devices(&mut self, devtype: DeviceType, collection: &DeviceCollectionRef) -> Result<()> {
+    fn enumerate_devices(
+        &mut self,
+        devtype: DeviceType,
+        collection: &DeviceCollectionRef,
+    ) -> Result<()> {
         assert_not_in_callback();
         let v: Vec<ffi::cubeb_device_info> = match send_recv!(self.rpc(),
                              ContextGetDeviceEnumeration(devtype.bits()) =>
@@ -286,7 +304,11 @@ impl ContextOps for ClientContext {
         assert_not_in_callback();
         unsafe {
             let coll = &mut *collection.as_ptr();
-            let mut devices = Vec::from_raw_parts(coll.device as *mut ffi::cubeb_device_info, coll.count, coll.count);
+            let mut devices = Vec::from_raw_parts(
+                coll.device as *mut ffi::cubeb_device_info,
+                coll.count,
+                coll.count,
+            );
             for dev in &mut devices {
                 if !dev.device_id.is_null() {
                     let _ = CString::from_raw(dev.device_id as *mut _);
@@ -361,7 +383,8 @@ impl ContextOps for ClientContext {
                                  ContextSetupDeviceCollectionCallback =>
                                  ContextSetupDeviceCollectionCallback())?;
 
-            let stream = unsafe { audioipc::MessageStream::from_raw_fd(fds.platform_handles[0].into_raw()) };
+            let stream =
+                unsafe { audioipc::MessageStream::from_raw_fd(fds.platform_handles[0].into_raw()) };
 
             // TODO: The lowest comms layer expects exactly 3 PlatformHandles, but we only
             // need one here.  Drop the dummy handles the other side sent us to discard.
