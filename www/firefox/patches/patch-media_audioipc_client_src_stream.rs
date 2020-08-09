$NetBSD$

--- media/audioipc/client/src/stream.rs.orig	Mon Jul 20 22:49:41 2020
+++ media/audioipc/client/src/stream.rs
@@ -62,7 +62,8 @@ impl rpc::Server for CallbackServer {
     type Request = CallbackReq;
     type Response = CallbackResp;
     type Future = CpuFuture<Self::Response, ()>;
-    type Transport = Framed<audioipc::AsyncMessageStream, LengthDelimitedCodec<Self::Response, Self::Request>>;
+    type Transport =
+        Framed<audioipc::AsyncMessageStream, LengthDelimitedCodec<Self::Response, Self::Request>>;
 
     fn process(&mut self, req: Self::Request) -> Self::Future {
         match req {
@@ -93,7 +94,10 @@ impl rpc::Server for CallbackServer {
                 self.cpu_pool.spawn_fn(move || {
                     // TODO: This is proof-of-concept. Make it better.
                     let input_ptr: *const u8 = match input_shm {
-                        Some(shm) => shm.get_slice(nframes as usize * input_frame_size).unwrap().as_ptr(),
+                        Some(shm) => shm
+                            .get_slice(nframes as usize * input_frame_size)
+                            .unwrap()
+                            .as_ptr(),
                         None => ptr::null(),
                     };
                     let output_ptr: *mut u8 = match output_shm {
@@ -169,9 +173,13 @@ impl<'ctx> ClientStream<'ctx> {
         let rpc = ctx.rpc();
         let data = send_recv!(rpc, StreamInit(init_params) => StreamCreated())?;
 
-        debug!("token = {}, handles = {:?}", data.token, data.platform_handles);
+        debug!(
+            "token = {}, handles = {:?}",
+            data.token, data.platform_handles
+        );
 
-        let stream = unsafe { audioipc::MessageStream::from_raw_fd(data.platform_handles[0].into_raw()) };
+        let stream =
+            unsafe { audioipc::MessageStream::from_raw_fd(data.platform_handles[0].into_raw()) };
 
         let input_file = unsafe { data.platform_handles[1].into_file() };
         let input_shm = if has_input {
