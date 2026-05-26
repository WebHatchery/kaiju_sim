// WASM browser bridge for macroquad-toolkit persistence.
// Loaded after mq_js_bundle.js and sapp_jsutils.js, before load(...).
(function() {
    if (typeof plugins !== "undefined" && Array.isArray(plugins)) {
        plugins = plugins.filter(function(plugin) {
            return !plugin || (plugin.name !== "macroquad_audio" && plugin.name !== "quad_net");
        });
    }

    if (typeof importObject === "undefined" || !importObject.env) {
        console.error("storage bridge failed: Miniquad importObject is unavailable");
        return;
    }

    importObject.env.storage_set_extern = function(key_obj, value_obj) {
        var key = consume_js_object(key_obj);
        var value = consume_js_object(value_obj);
        try {
            localStorage.setItem(key, value);
        } catch (e) {
            console.error("storage_set failed:", e);
        }
    };

    importObject.env.storage_get_extern = function(key_obj) {
        var key = consume_js_object(key_obj);
        try {
            var value = localStorage.getItem(key);
            if (value === null) {
                return -1;
            }
            return js_object(value);
        } catch (e) {
            console.error("storage_get failed:", e);
            return -1;
        }
    };

    importObject.env.storage_remove_extern = function(key_obj) {
        var key = consume_js_object(key_obj);
        try {
            localStorage.removeItem(key);
        } catch (e) {
            console.error("storage_remove failed:", e);
        }
    };

    importObject.env.storage_exists_extern = function(key_obj) {
        var key = consume_js_object(key_obj);
        try {
            return localStorage.getItem(key) !== null;
        } catch (e) {
            return false;
        }
    };

    importObject.env.glFramebufferRenderbuffer = function(target, attachment, renderbuffertarget, renderbuffer) {
        GL.validateGLObjectID(GL.renderbuffers, renderbuffer, "glFramebufferRenderbuffer", "renderbuffer");
        gl.framebufferRenderbuffer(target, attachment, renderbuffertarget, GL.renderbuffers[renderbuffer]);
    };

    importObject.env.glCheckFramebufferStatus = function(target) {
        return gl.checkFramebufferStatus(target);
    };

    importObject.env.glRenderbufferStorageMultisample = function(target, samples, internalformat, width, height) {
        if (gl.renderbufferStorageMultisample) {
            gl.renderbufferStorageMultisample(target, samples, internalformat, width, height);
        } else {
            gl.renderbufferStorage(target, internalformat, width, height);
        }
    };

    importObject.env.glDeleteRenderbuffers = function(count, renderbuffers) {
        for (var i = 0; i < count; i++) {
            var id = getArray(renderbuffers + i * 4, Uint32Array, 1)[0];
            var renderbuffer = GL.renderbuffers[id];
            if (!renderbuffer) {
                continue;
            }
            gl.deleteRenderbuffer(renderbuffer);
            renderbuffer.name = 0;
            GL.renderbuffers[id] = null;
        }
    };

    importObject.env.glReadBuffer = function(src) {
        if (gl.readBuffer) {
            gl.readBuffer(src);
        }
    };

    importObject.env.glBlitFramebuffer = function(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) {
        if (gl.blitFramebuffer) {
            gl.blitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
        }
    };
})();
