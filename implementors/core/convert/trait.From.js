(function() {var implementors = {};
implementors["bytes"] = [{"text":"impl&lt;'a&gt; From&lt;&amp;'a mut [u8]&gt; for IoSliceMut&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a mut [MaybeUninit&lt;u8&gt;]&gt; for IoSliceMut&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl From&lt;&amp;'static [u8]&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl From&lt;&amp;'static str&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl From&lt;Vec&lt;u8&gt;&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl From&lt;String&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a [u8]&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; From&lt;&amp;'a str&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl From&lt;BytesMut&gt; for Bytes","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl&lt;T&gt; From&lt;(T, TrySendError)&gt; for SendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;(T, TrySendError)&gt; for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;SendError&lt;T&gt;&gt; for TrySendError&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for Mutex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; From&lt;T&gt; for RwLock&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()