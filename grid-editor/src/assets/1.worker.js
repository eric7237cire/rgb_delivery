self.webpackChunk([1],{4:function(n,t,r){"use strict";r.r(t),r.d(t,"__wbg_alert_10aa8795b4e16438",function(){return f}),r.d(t,"greet",function(){return l}),r.d(t,"get_colors",function(){return g}),r.d(t,"__wbg_error_cc95a3d302735ca3",function(){return j}),r.d(t,"__widl_f_log_1_",function(){return x}),r.d(t,"__wbindgen_object_drop_ref",function(){return k}),r.d(t,"__wbindgen_string_new",function(){return E}),r.d(t,"__wbindgen_json_parse",function(){return O}),r.d(t,"__wbindgen_json_serialize",function(){return N}),r.d(t,"Universe",function(){return S}),r.d(t,"__wbindgen_throw",function(){return T});var e=r(5);let u=new TextDecoder("utf-8"),o=null;function i(){return null!==o&&o.buffer===e.g.buffer||(o=new Uint8Array(e.g.buffer)),o}function c(n,t){return u.decode(i().subarray(n,n+t))}function f(n,t){let r=c(n,t);alert(r)}function l(){return e.f()}const s=new Array(32);function _(n){return s[n]}s.fill(void 0),s.push(void 0,null,!0,!1);let a=s.length;function d(n){n<36||(s[n]=a,a=n)}function w(n){const t=_(n);return d(n),t}function g(){return w(e.e())}let h=null;let p=null;function b(){return null!==p&&p.buffer===e.g.buffer||(p=new Uint32Array(e.g.buffer)),p}let y=32;function v(n){if(1==y)throw new Error("out of js stack");return s[--y]=n,y}function j(n,t){let r=c(n,t);r=r.slice(),e.b(n,1*t),console.error(r)}function x(n){console.log(_(n))}function k(n){d(n)}function A(n){a===s.length&&s.push(s.length+1);const t=a;return a=s[t],s[t]=n,t}function E(n,t){return A(c(n,t))}function O(n,t){return A(JSON.parse(c(n,t)))}let U=new TextEncoder("utf-8"),J=0;function N(n,t){const r=function(n){const t=U.encode(n),r=e.d(t.length);return i().set(t,r),J=t.length,r}(JSON.stringify(_(n)));return b()[t/4]=r,J}class S{static __wrap(n){const t=Object.create(S.prototype);return t.ptr=n,t}free(){const n=this.ptr;this.ptr=0,function(n){e.a(n)}(n)}static new(n,t){return S.__wrap(e.j(n,t))}render(){const n=(null===h&&(h=e.c()),h);e.m(n,this.ptr);const t=b(),r=t[n/4],u=t[n/4+1],o=c(r,u).slice();return e.b(r,1*u),o}get_data(){return w(e.h(this.ptr))}set_overrides(n){try{return e.n(this.ptr,v(n))}finally{s[y++]=void 0}}init_calculate(){return e.i(this.ptr)}next_calculate(){return w(e.l(this.ptr))}next_batch_calculate(n){return w(e.k(this.ptr,n))}set_square(n){try{return e.o(this.ptr,v(n))}finally{s[y++]=void 0}}}function T(n,t){throw new Error(c(n,t))}},5:function(n,t,r){"use strict";var e=r.w[n.i];n.exports=e;r(4);e.p()}});
//# sourceMappingURL=1.worker.js.map