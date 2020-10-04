(function() {var implementors = {};
implementors["bytes"] = [{"text":"impl UnwindSafe for Bytes","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BytesMut","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !UnwindSafe for IoSliceMut&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for IntoIter&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, U&gt; UnwindSafe for Chain&lt;T, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Limit&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for Take&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B&gt; UnwindSafe for Reader&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;B&gt; UnwindSafe for Writer&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["fnv"] = [{"text":"impl UnwindSafe for FnvHasher","synthetic":true,"types":[]}];
implementors["lastshot"] = [{"text":"impl&lt;T&gt; UnwindSafe for NoReceivers&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for NoSenders","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]}];
implementors["tokio"] = [{"text":"impl !UnwindSafe for Builder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Handle","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryCurrentError","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Runtime","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Barrier","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BarrierWaitResult","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized&gt; UnwindSafe for Mutex&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for MutexGuard&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryLockError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for OwnedMutexGuard&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Notify","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Semaphore","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !UnwindSafe for SemaphorePermit&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for OwnedSemaphorePermit","synthetic":true,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized&gt; UnwindSafe for RwLock&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for RwLockReadGuard&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; !UnwindSafe for RwLockWriteGuard&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for SendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecvError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryRecvError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for UnboundedReceiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for UnboundedSender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for SendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecvError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ClosedError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for TrySendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryRecvError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecvError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TryRecvError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Receiver&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; !UnwindSafe for Sender&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; UnwindSafe for Ref&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for SendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()