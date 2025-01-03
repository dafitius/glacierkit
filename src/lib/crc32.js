!(function (r) {
	if ("object" == typeof exports && "undefined" != typeof module) module.exports = r()
	else if ("function" == typeof define && define.amd) define([], r)
	else {
		;("undefined" != typeof window ? window : "undefined" != typeof global ? global : "undefined" != typeof self ? self : this).crc = r()
	}
})(function () {
	return (function () {
		return function r(t, e, n) {
			function f(u, o) {
				if (!e[u]) {
					if (!t[u]) {
						var s = "function" == typeof require && require
						if (!o && s) return s(u, !0)
						if (i) return i(u, !0)
						var a = new Error("Cannot find module '" + u + "'")
						throw ((a.code = "MODULE_NOT_FOUND"), a)
					}
					var c = (e[u] = { exports: {} })
					t[u][0].call(
						c.exports,
						function (r) {
							return f(t[u][1][r] || r)
						},
						c,
						c.exports,
						r,
						t,
						e,
						n
					)
				}
				return e[u].exports
			}
			for (var i = "function" == typeof require && require, u = 0; u < n.length; u++) f(n[u])
			return f
		}
	})()(
		{
			1: [
				function (r, t, e) {
					"use strict"
					;(e.byteLength = function (r) {
						var t = a(r),
							e = t[0],
							n = t[1]
						return (3 * (e + n)) / 4 - n
					}),
						(e.toByteArray = function (r) {
							for (
								var t,
									e = a(r),
									n = e[0],
									u = e[1],
									o = new i(
										(function (r, t, e) {
											return (3 * (t + e)) / 4 - e
										})(0, n, u)
									),
									s = 0,
									c = u > 0 ? n - 4 : n,
									h = 0;
								h < c;
								h += 4
							)
								(t = (f[r.charCodeAt(h)] << 18) | (f[r.charCodeAt(h + 1)] << 12) | (f[r.charCodeAt(h + 2)] << 6) | f[r.charCodeAt(h + 3)]),
									(o[s++] = (t >> 16) & 255),
									(o[s++] = (t >> 8) & 255),
									(o[s++] = 255 & t)
							2 === u && ((t = (f[r.charCodeAt(h)] << 2) | (f[r.charCodeAt(h + 1)] >> 4)), (o[s++] = 255 & t))
							1 === u && ((t = (f[r.charCodeAt(h)] << 10) | (f[r.charCodeAt(h + 1)] << 4) | (f[r.charCodeAt(h + 2)] >> 2)), (o[s++] = (t >> 8) & 255), (o[s++] = 255 & t))
							return o
						}),
						(e.fromByteArray = function (r) {
							for (var t, e = r.length, f = e % 3, i = [], u = 0, o = e - f; u < o; u += 16383) i.push(c(r, u, u + 16383 > o ? o : u + 16383))
							1 === f
								? ((t = r[e - 1]), i.push(n[t >> 2] + n[(t << 4) & 63] + "=="))
								: 2 === f && ((t = (r[e - 2] << 8) + r[e - 1]), i.push(n[t >> 10] + n[(t >> 4) & 63] + n[(t << 2) & 63] + "="))
							return i.join("")
						})
					for (
						var n = [], f = [], i = "undefined" != typeof Uint8Array ? Uint8Array : Array, u = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", o = 0, s = u.length;
						o < s;
						++o
					)
						(n[o] = u[o]), (f[u.charCodeAt(o)] = o)
					function a(r) {
						var t = r.length
						if (t % 4 > 0) throw new Error("Invalid string. Length must be a multiple of 4")
						var e = r.indexOf("=")
						return -1 === e && (e = t), [e, e === t ? 0 : 4 - (e % 4)]
					}
					function c(r, t, e) {
						for (var f, i, u = [], o = t; o < e; o += 3)
							(f = ((r[o] << 16) & 16711680) + ((r[o + 1] << 8) & 65280) + (255 & r[o + 2])), u.push(n[((i = f) >> 18) & 63] + n[(i >> 12) & 63] + n[(i >> 6) & 63] + n[63 & i])
						return u.join("")
					}
					;(f["-".charCodeAt(0)] = 62), (f["_".charCodeAt(0)] = 63)
				},
				{}
			],
			2: [
				function (r, t, e) {
					"use strict"
					var n = r("base64-js"),
						f = r("ieee754")
					;(e.Buffer = o),
						(e.SlowBuffer = function (r) {
							;+r != r && (r = 0)
							return o.alloc(+r)
						}),
						(e.INSPECT_MAX_BYTES = 50)
					var i = 2147483647
					function u(r) {
						if (r > i) throw new RangeError('The value "' + r + '" is invalid for option "size"')
						var t = new Uint8Array(r)
						return (t.__proto__ = o.prototype), t
					}
					function o(r, t, e) {
						if ("number" == typeof r) {
							if ("string" == typeof t) throw new TypeError('The "string" argument must be of type string. Received type number')
							return c(r)
						}
						return s(r, t, e)
					}
					function s(r, t, e) {
						if ("string" == typeof r)
							return (function (r, t) {
								;("string" == typeof t && "" !== t) || (t = "utf8")
								if (!o.isEncoding(t)) throw new TypeError("Unknown encoding: " + t)
								var e = 0 | p(r, t),
									n = u(e),
									f = n.write(r, t)
								f !== e && (n = n.slice(0, f))
								return n
							})(r, t)
						if (ArrayBuffer.isView(r)) return h(r)
						if (null == r) throw TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type " + typeof r)
						if (Y(r, ArrayBuffer) || (r && Y(r.buffer, ArrayBuffer)))
							return (function (r, t, e) {
								if (t < 0 || r.byteLength < t) throw new RangeError('"offset" is outside of buffer bounds')
								if (r.byteLength < t + (e || 0)) throw new RangeError('"length" is outside of buffer bounds')
								var n
								n = void 0 === t && void 0 === e ? new Uint8Array(r) : void 0 === e ? new Uint8Array(r, t) : new Uint8Array(r, t, e)
								return (n.__proto__ = o.prototype), n
							})(r, t, e)
						if ("number" == typeof r) throw new TypeError('The "value" argument must not be of type number. Received type number')
						var n = r.valueOf && r.valueOf()
						if (null != n && n !== r) return o.from(n, t, e)
						var f = (function (r) {
							if (o.isBuffer(r)) {
								var t = 0 | l(r.length),
									e = u(t)
								return 0 === e.length ? e : (r.copy(e, 0, 0, t), e)
							}
							if (void 0 !== r.length) return "number" != typeof r.length || P(r.length) ? u(0) : h(r)
							if ("Buffer" === r.type && Array.isArray(r.data)) return h(r.data)
						})(r)
						if (f) return f
						if ("undefined" != typeof Symbol && null != Symbol.toPrimitive && "function" == typeof r[Symbol.toPrimitive]) return o.from(r[Symbol.toPrimitive]("string"), t, e)
						throw new TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type " + typeof r)
					}
					function a(r) {
						if ("number" != typeof r) throw new TypeError('"size" argument must be of type number')
						if (r < 0) throw new RangeError('The value "' + r + '" is invalid for option "size"')
					}
					function c(r) {
						return a(r), u(r < 0 ? 0 : 0 | l(r))
					}
					function h(r) {
						for (var t = r.length < 0 ? 0 : 0 | l(r.length), e = u(t), n = 0; n < t; n += 1) e[n] = 255 & r[n]
						return e
					}
					function l(r) {
						if (r >= i) throw new RangeError("Attempt to allocate Buffer larger than maximum size: 0x" + i.toString(16) + " bytes")
						return 0 | r
					}
					function p(r, t) {
						if (o.isBuffer(r)) return r.length
						if (ArrayBuffer.isView(r) || Y(r, ArrayBuffer)) return r.byteLength
						if ("string" != typeof r) throw new TypeError('The "string" argument must be one of type string, Buffer, or ArrayBuffer. Received type ' + typeof r)
						var e = r.length,
							n = arguments.length > 2 && !0 === arguments[2]
						if (!n && 0 === e) return 0
						for (var f = !1; ; )
							switch (t) {
								case "ascii":
								case "latin1":
								case "binary":
									return e
								case "utf8":
								case "utf-8":
									return D(r).length
								case "ucs2":
								case "ucs-2":
								case "utf16le":
								case "utf-16le":
									return 2 * e
								case "hex":
									return e >>> 1
								case "base64":
									return q(r).length
								default:
									if (f) return n ? -1 : D(r).length
									;(t = ("" + t).toLowerCase()), (f = !0)
							}
					}
					function y(r, t, e) {
						var n = r[t]
						;(r[t] = r[e]), (r[e] = n)
					}
					function v(r, t, e, n, f) {
						if (0 === r.length) return -1
						if (
							("string" == typeof e ? ((n = e), (e = 0)) : e > 2147483647 ? (e = 2147483647) : e < -2147483648 && (e = -2147483648),
							P((e = +e)) && (e = f ? 0 : r.length - 1),
							e < 0 && (e = r.length + e),
							e >= r.length)
						) {
							if (f) return -1
							e = r.length - 1
						} else if (e < 0) {
							if (!f) return -1
							e = 0
						}
						if (("string" == typeof t && (t = o.from(t, n)), o.isBuffer(t))) return 0 === t.length ? -1 : g(r, t, e, n, f)
						if ("number" == typeof t)
							return (
								(t &= 255),
								"function" == typeof Uint8Array.prototype.indexOf
									? f
										? Uint8Array.prototype.indexOf.call(r, t, e)
										: Uint8Array.prototype.lastIndexOf.call(r, t, e)
									: g(r, [t], e, n, f)
							)
						throw new TypeError("val must be string, number or Buffer")
					}
					function g(r, t, e, n, f) {
						var i,
							u = 1,
							o = r.length,
							s = t.length
						if (void 0 !== n && ("ucs2" === (n = String(n).toLowerCase()) || "ucs-2" === n || "utf16le" === n || "utf-16le" === n)) {
							if (r.length < 2 || t.length < 2) return -1
							;(u = 2), (o /= 2), (s /= 2), (e /= 2)
						}
						function a(r, t) {
							return 1 === u ? r[t] : r.readUInt16BE(t * u)
						}
						if (f) {
							var c = -1
							for (i = e; i < o; i++)
								if (a(r, i) === a(t, -1 === c ? 0 : i - c)) {
									if ((-1 === c && (c = i), i - c + 1 === s)) return c * u
								} else -1 !== c && (i -= i - c), (c = -1)
						} else
							for (e + s > o && (e = o - s), i = e; i >= 0; i--) {
								for (var h = !0, l = 0; l < s; l++)
									if (a(r, i + l) !== a(t, l)) {
										h = !1
										break
									}
								if (h) return i
							}
						return -1
					}
					function w(r, t, e, n) {
						e = Number(e) || 0
						var f = r.length - e
						n ? (n = Number(n)) > f && (n = f) : (n = f)
						var i = t.length
						n > i / 2 && (n = i / 2)
						for (var u = 0; u < n; ++u) {
							var o = parseInt(t.substr(2 * u, 2), 16)
							if (P(o)) return u
							r[e + u] = o
						}
						return u
					}
					function b(r, t, e, n) {
						return F(D(t, r.length - e), r, e, n)
					}
					function _(r, t, e, n) {
						return F(
							(function (r) {
								for (var t = [], e = 0; e < r.length; ++e) t.push(255 & r.charCodeAt(e))
								return t
							})(t),
							r,
							e,
							n
						)
					}
					function m(r, t, e, n) {
						return _(r, t, e, n)
					}
					function d(r, t, e, n) {
						return F(q(t), r, e, n)
					}
					function B(r, t, e, n) {
						return F(
							(function (r, t) {
								for (var e, n, f, i = [], u = 0; u < r.length && !((t -= 2) < 0); ++u) (e = r.charCodeAt(u)), (n = e >> 8), (f = e % 256), i.push(f), i.push(n)
								return i
							})(t, r.length - e),
							r,
							e,
							n
						)
					}
					function A(r, t, e) {
						return 0 === t && e === r.length ? n.fromByteArray(r) : n.fromByteArray(r.slice(t, e))
					}
					function E(r, t, e) {
						e = Math.min(r.length, e)
						for (var n = [], f = t; f < e; ) {
							var i,
								u,
								o,
								s,
								a = r[f],
								c = null,
								h = a > 239 ? 4 : a > 223 ? 3 : a > 191 ? 2 : 1
							if (f + h <= e)
								switch (h) {
									case 1:
										a < 128 && (c = a)
										break
									case 2:
										128 == (192 & (i = r[f + 1])) && (s = ((31 & a) << 6) | (63 & i)) > 127 && (c = s)
										break
									case 3:
										;(i = r[f + 1]),
											(u = r[f + 2]),
											128 == (192 & i) && 128 == (192 & u) && (s = ((15 & a) << 12) | ((63 & i) << 6) | (63 & u)) > 2047 && (s < 55296 || s > 57343) && (c = s)
										break
									case 4:
										;(i = r[f + 1]),
											(u = r[f + 2]),
											(o = r[f + 3]),
											128 == (192 & i) &&
												128 == (192 & u) &&
												128 == (192 & o) &&
												(s = ((15 & a) << 18) | ((63 & i) << 12) | ((63 & u) << 6) | (63 & o)) > 65535 &&
												s < 1114112 &&
												(c = s)
								}
							null === c ? ((c = 65533), (h = 1)) : c > 65535 && ((c -= 65536), n.push(((c >>> 10) & 1023) | 55296), (c = 56320 | (1023 & c))), n.push(c), (f += h)
						}
						return (function (r) {
							var t = r.length
							if (t <= M) return String.fromCharCode.apply(String, r)
							var e = "",
								n = 0
							for (; n < t; ) e += String.fromCharCode.apply(String, r.slice(n, (n += M)))
							return e
						})(n)
					}
					;(e.kMaxLength = i),
						(o.TYPED_ARRAY_SUPPORT = (function () {
							try {
								var r = new Uint8Array(1)
								return (
									(r.__proto__ = {
										__proto__: Uint8Array.prototype,
										foo: function () {
											return 42
										}
									}),
									42 === r.foo()
								)
							} catch (r) {
								return !1
							}
						})()),
						o.TYPED_ARRAY_SUPPORT ||
							"undefined" == typeof console ||
							"function" != typeof console.error ||
							console.error("This browser lacks typed array (Uint8Array) support which is required by `buffer` v5.x. Use `buffer` v4.x if you require old browser support."),
						Object.defineProperty(o.prototype, "parent", {
							enumerable: !0,
							get: function () {
								if (o.isBuffer(this)) return this.buffer
							}
						}),
						Object.defineProperty(o.prototype, "offset", {
							enumerable: !0,
							get: function () {
								if (o.isBuffer(this)) return this.byteOffset
							}
						}),
						"undefined" != typeof Symbol &&
							null != Symbol.species &&
							o[Symbol.species] === o &&
							Object.defineProperty(o, Symbol.species, { value: null, configurable: !0, enumerable: !1, writable: !1 }),
						(o.poolSize = 8192),
						(o.from = function (r, t, e) {
							return s(r, t, e)
						}),
						(o.prototype.__proto__ = Uint8Array.prototype),
						(o.__proto__ = Uint8Array),
						(o.alloc = function (r, t, e) {
							return (function (r, t, e) {
								return a(r), r <= 0 ? u(r) : void 0 !== t ? ("string" == typeof e ? u(r).fill(t, e) : u(r).fill(t)) : u(r)
							})(r, t, e)
						}),
						(o.allocUnsafe = function (r) {
							return c(r)
						}),
						(o.allocUnsafeSlow = function (r) {
							return c(r)
						}),
						(o.isBuffer = function (r) {
							return null != r && !0 === r._isBuffer && r !== o.prototype
						}),
						(o.compare = function (r, t) {
							if ((Y(r, Uint8Array) && (r = o.from(r, r.offset, r.byteLength)), Y(t, Uint8Array) && (t = o.from(t, t.offset, t.byteLength)), !o.isBuffer(r) || !o.isBuffer(t)))
								throw new TypeError('The "buf1", "buf2" arguments must be one of type Buffer or Uint8Array')
							if (r === t) return 0
							for (var e = r.length, n = t.length, f = 0, i = Math.min(e, n); f < i; ++f)
								if (r[f] !== t[f]) {
									;(e = r[f]), (n = t[f])
									break
								}
							return e < n ? -1 : n < e ? 1 : 0
						}),
						(o.isEncoding = function (r) {
							switch (String(r).toLowerCase()) {
								case "hex":
								case "utf8":
								case "utf-8":
								case "ascii":
								case "latin1":
								case "binary":
								case "base64":
								case "ucs2":
								case "ucs-2":
								case "utf16le":
								case "utf-16le":
									return !0
								default:
									return !1
							}
						}),
						(o.concat = function (r, t) {
							if (!Array.isArray(r)) throw new TypeError('"list" argument must be an Array of Buffers')
							if (0 === r.length) return o.alloc(0)
							var e
							if (void 0 === t) for (t = 0, e = 0; e < r.length; ++e) t += r[e].length
							var n = o.allocUnsafe(t),
								f = 0
							for (e = 0; e < r.length; ++e) {
								var i = r[e]
								if ((Y(i, Uint8Array) && (i = o.from(i)), !o.isBuffer(i))) throw new TypeError('"list" argument must be an Array of Buffers')
								i.copy(n, f), (f += i.length)
							}
							return n
						}),
						(o.byteLength = p),
						(o.prototype._isBuffer = !0),
						(o.prototype.swap16 = function () {
							var r = this.length
							if (r % 2 != 0) throw new RangeError("Buffer size must be a multiple of 16-bits")
							for (var t = 0; t < r; t += 2) y(this, t, t + 1)
							return this
						}),
						(o.prototype.swap32 = function () {
							var r = this.length
							if (r % 4 != 0) throw new RangeError("Buffer size must be a multiple of 32-bits")
							for (var t = 0; t < r; t += 4) y(this, t, t + 3), y(this, t + 1, t + 2)
							return this
						}),
						(o.prototype.swap64 = function () {
							var r = this.length
							if (r % 8 != 0) throw new RangeError("Buffer size must be a multiple of 64-bits")
							for (var t = 0; t < r; t += 8) y(this, t, t + 7), y(this, t + 1, t + 6), y(this, t + 2, t + 5), y(this, t + 3, t + 4)
							return this
						}),
						(o.prototype.toString = function () {
							var r = this.length
							return 0 === r
								? ""
								: 0 === arguments.length
									? E(this, 0, r)
									: function (r, t, e) {
											var n = !1
											if (((void 0 === t || t < 0) && (t = 0), t > this.length)) return ""
											if (((void 0 === e || e > this.length) && (e = this.length), e <= 0)) return ""
											if ((e >>>= 0) <= (t >>>= 0)) return ""
											for (r || (r = "utf8"); ; )
												switch (r) {
													case "hex":
														return C(this, t, e)
													case "utf8":
													case "utf-8":
														return E(this, t, e)
													case "ascii":
														return U(this, t, e)
													case "latin1":
													case "binary":
														return T(this, t, e)
													case "base64":
														return A(this, t, e)
													case "ucs2":
													case "ucs-2":
													case "utf16le":
													case "utf-16le":
														return R(this, t, e)
													default:
														if (n) throw new TypeError("Unknown encoding: " + r)
														;(r = (r + "").toLowerCase()), (n = !0)
												}
										}.apply(this, arguments)
						}),
						(o.prototype.toLocaleString = o.prototype.toString),
						(o.prototype.equals = function (r) {
							if (!o.isBuffer(r)) throw new TypeError("Argument must be a Buffer")
							return this === r || 0 === o.compare(this, r)
						}),
						(o.prototype.inspect = function () {
							var r = "",
								t = e.INSPECT_MAX_BYTES
							return (
								(r = this.toString("hex", 0, t)
									.replace(/(.{2})/g, "$1 ")
									.trim()),
								this.length > t && (r += " ... "),
								"<Buffer " + r + ">"
							)
						}),
						(o.prototype.compare = function (r, t, e, n, f) {
							if ((Y(r, Uint8Array) && (r = o.from(r, r.offset, r.byteLength)), !o.isBuffer(r)))
								throw new TypeError('The "target" argument must be one of type Buffer or Uint8Array. Received type ' + typeof r)
							if (
								(void 0 === t && (t = 0),
								void 0 === e && (e = r ? r.length : 0),
								void 0 === n && (n = 0),
								void 0 === f && (f = this.length),
								t < 0 || e > r.length || n < 0 || f > this.length)
							)
								throw new RangeError("out of range index")
							if (n >= f && t >= e) return 0
							if (n >= f) return -1
							if (t >= e) return 1
							if (this === r) return 0
							for (var i = (f >>>= 0) - (n >>>= 0), u = (e >>>= 0) - (t >>>= 0), s = Math.min(i, u), a = this.slice(n, f), c = r.slice(t, e), h = 0; h < s; ++h)
								if (a[h] !== c[h]) {
									;(i = a[h]), (u = c[h])
									break
								}
							return i < u ? -1 : u < i ? 1 : 0
						}),
						(o.prototype.includes = function (r, t, e) {
							return -1 !== this.indexOf(r, t, e)
						}),
						(o.prototype.indexOf = function (r, t, e) {
							return v(this, r, t, e, !0)
						}),
						(o.prototype.lastIndexOf = function (r, t, e) {
							return v(this, r, t, e, !1)
						}),
						(o.prototype.write = function (r, t, e, n) {
							if (void 0 === t) (n = "utf8"), (e = this.length), (t = 0)
							else if (void 0 === e && "string" == typeof t) (n = t), (e = this.length), (t = 0)
							else {
								if (!isFinite(t)) throw new Error("Buffer.write(string, encoding, offset[, length]) is no longer supported")
								;(t >>>= 0), isFinite(e) ? ((e >>>= 0), void 0 === n && (n = "utf8")) : ((n = e), (e = void 0))
							}
							var f = this.length - t
							if (((void 0 === e || e > f) && (e = f), (r.length > 0 && (e < 0 || t < 0)) || t > this.length)) throw new RangeError("Attempt to write outside buffer bounds")
							n || (n = "utf8")
							for (var i = !1; ; )
								switch (n) {
									case "hex":
										return w(this, r, t, e)
									case "utf8":
									case "utf-8":
										return b(this, r, t, e)
									case "ascii":
										return _(this, r, t, e)
									case "latin1":
									case "binary":
										return m(this, r, t, e)
									case "base64":
										return d(this, r, t, e)
									case "ucs2":
									case "ucs-2":
									case "utf16le":
									case "utf-16le":
										return B(this, r, t, e)
									default:
										if (i) throw new TypeError("Unknown encoding: " + n)
										;(n = ("" + n).toLowerCase()), (i = !0)
								}
						}),
						(o.prototype.toJSON = function () {
							return { type: "Buffer", data: Array.prototype.slice.call(this._arr || this, 0) }
						})
					var M = 4096
					function U(r, t, e) {
						var n = ""
						e = Math.min(r.length, e)
						for (var f = t; f < e; ++f) n += String.fromCharCode(127 & r[f])
						return n
					}
					function T(r, t, e) {
						var n = ""
						e = Math.min(r.length, e)
						for (var f = t; f < e; ++f) n += String.fromCharCode(r[f])
						return n
					}
					function C(r, t, e) {
						var n = r.length
						;(!t || t < 0) && (t = 0), (!e || e < 0 || e > n) && (e = n)
						for (var f = "", i = t; i < e; ++i) f += N(r[i])
						return f
					}
					function R(r, t, e) {
						for (var n = r.slice(t, e), f = "", i = 0; i < n.length; i += 2) f += String.fromCharCode(n[i] + 256 * n[i + 1])
						return f
					}
					function O(r, t, e) {
						if (r % 1 != 0 || r < 0) throw new RangeError("offset is not uint")
						if (r + t > e) throw new RangeError("Trying to access beyond buffer length")
					}
					function j(r, t, e, n, f, i) {
						if (!o.isBuffer(r)) throw new TypeError('"buffer" argument must be a Buffer instance')
						if (t > f || t < i) throw new RangeError('"value" argument is out of bounds')
						if (e + n > r.length) throw new RangeError("Index out of range")
					}
					function k(r, t, e, n, f, i) {
						if (e + n > r.length) throw new RangeError("Index out of range")
						if (e < 0) throw new RangeError("Index out of range")
					}
					function L(r, t, e, n, i) {
						return (t = +t), (e >>>= 0), i || k(r, 0, e, 4), f.write(r, t, e, n, 23, 4), e + 4
					}
					function S(r, t, e, n, i) {
						return (t = +t), (e >>>= 0), i || k(r, 0, e, 8), f.write(r, t, e, n, 52, 8), e + 8
					}
					;(o.prototype.slice = function (r, t) {
						var e = this.length
						;(r = ~~r) < 0 ? (r += e) < 0 && (r = 0) : r > e && (r = e), (t = void 0 === t ? e : ~~t) < 0 ? (t += e) < 0 && (t = 0) : t > e && (t = e), t < r && (t = r)
						var n = this.subarray(r, t)
						return (n.__proto__ = o.prototype), n
					}),
						(o.prototype.readUIntLE = function (r, t, e) {
							;(r >>>= 0), (t >>>= 0), e || O(r, t, this.length)
							for (var n = this[r], f = 1, i = 0; ++i < t && (f *= 256); ) n += this[r + i] * f
							return n
						}),
						(o.prototype.readUIntBE = function (r, t, e) {
							;(r >>>= 0), (t >>>= 0), e || O(r, t, this.length)
							for (var n = this[r + --t], f = 1; t > 0 && (f *= 256); ) n += this[r + --t] * f
							return n
						}),
						(o.prototype.readUInt8 = function (r, t) {
							return (r >>>= 0), t || O(r, 1, this.length), this[r]
						}),
						(o.prototype.readUInt16LE = function (r, t) {
							return (r >>>= 0), t || O(r, 2, this.length), this[r] | (this[r + 1] << 8)
						}),
						(o.prototype.readUInt16BE = function (r, t) {
							return (r >>>= 0), t || O(r, 2, this.length), (this[r] << 8) | this[r + 1]
						}),
						(o.prototype.readUInt32LE = function (r, t) {
							return (r >>>= 0), t || O(r, 4, this.length), (this[r] | (this[r + 1] << 8) | (this[r + 2] << 16)) + 16777216 * this[r + 3]
						}),
						(o.prototype.readUInt32BE = function (r, t) {
							return (r >>>= 0), t || O(r, 4, this.length), 16777216 * this[r] + ((this[r + 1] << 16) | (this[r + 2] << 8) | this[r + 3])
						}),
						(o.prototype.readIntLE = function (r, t, e) {
							;(r >>>= 0), (t >>>= 0), e || O(r, t, this.length)
							for (var n = this[r], f = 1, i = 0; ++i < t && (f *= 256); ) n += this[r + i] * f
							return n >= (f *= 128) && (n -= Math.pow(2, 8 * t)), n
						}),
						(o.prototype.readIntBE = function (r, t, e) {
							;(r >>>= 0), (t >>>= 0), e || O(r, t, this.length)
							for (var n = t, f = 1, i = this[r + --n]; n > 0 && (f *= 256); ) i += this[r + --n] * f
							return i >= (f *= 128) && (i -= Math.pow(2, 8 * t)), i
						}),
						(o.prototype.readInt8 = function (r, t) {
							return (r >>>= 0), t || O(r, 1, this.length), 128 & this[r] ? -1 * (255 - this[r] + 1) : this[r]
						}),
						(o.prototype.readInt16LE = function (r, t) {
							;(r >>>= 0), t || O(r, 2, this.length)
							var e = this[r] | (this[r + 1] << 8)
							return 32768 & e ? 4294901760 | e : e
						}),
						(o.prototype.readInt16BE = function (r, t) {
							;(r >>>= 0), t || O(r, 2, this.length)
							var e = this[r + 1] | (this[r] << 8)
							return 32768 & e ? 4294901760 | e : e
						}),
						(o.prototype.readInt32LE = function (r, t) {
							return (r >>>= 0), t || O(r, 4, this.length), this[r] | (this[r + 1] << 8) | (this[r + 2] << 16) | (this[r + 3] << 24)
						}),
						(o.prototype.readInt32BE = function (r, t) {
							return (r >>>= 0), t || O(r, 4, this.length), (this[r] << 24) | (this[r + 1] << 16) | (this[r + 2] << 8) | this[r + 3]
						}),
						(o.prototype.readFloatLE = function (r, t) {
							return (r >>>= 0), t || O(r, 4, this.length), f.read(this, r, !0, 23, 4)
						}),
						(o.prototype.readFloatBE = function (r, t) {
							return (r >>>= 0), t || O(r, 4, this.length), f.read(this, r, !1, 23, 4)
						}),
						(o.prototype.readDoubleLE = function (r, t) {
							return (r >>>= 0), t || O(r, 8, this.length), f.read(this, r, !0, 52, 8)
						}),
						(o.prototype.readDoubleBE = function (r, t) {
							return (r >>>= 0), t || O(r, 8, this.length), f.read(this, r, !1, 52, 8)
						}),
						(o.prototype.writeUIntLE = function (r, t, e, n) {
							;((r = +r), (t >>>= 0), (e >>>= 0), n) || j(this, r, t, e, Math.pow(2, 8 * e) - 1, 0)
							var f = 1,
								i = 0
							for (this[t] = 255 & r; ++i < e && (f *= 256); ) this[t + i] = (r / f) & 255
							return t + e
						}),
						(o.prototype.writeUIntBE = function (r, t, e, n) {
							;((r = +r), (t >>>= 0), (e >>>= 0), n) || j(this, r, t, e, Math.pow(2, 8 * e) - 1, 0)
							var f = e - 1,
								i = 1
							for (this[t + f] = 255 & r; --f >= 0 && (i *= 256); ) this[t + f] = (r / i) & 255
							return t + e
						}),
						(o.prototype.writeUInt8 = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 1, 255, 0), (this[t] = 255 & r), t + 1
						}),
						(o.prototype.writeUInt16LE = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 2, 65535, 0), (this[t] = 255 & r), (this[t + 1] = r >>> 8), t + 2
						}),
						(o.prototype.writeUInt16BE = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 2, 65535, 0), (this[t] = r >>> 8), (this[t + 1] = 255 & r), t + 2
						}),
						(o.prototype.writeUInt32LE = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 4, 4294967295, 0), (this[t + 3] = r >>> 24), (this[t + 2] = r >>> 16), (this[t + 1] = r >>> 8), (this[t] = 255 & r), t + 4
						}),
						(o.prototype.writeUInt32BE = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 4, 4294967295, 0), (this[t] = r >>> 24), (this[t + 1] = r >>> 16), (this[t + 2] = r >>> 8), (this[t + 3] = 255 & r), t + 4
						}),
						(o.prototype.writeIntLE = function (r, t, e, n) {
							if (((r = +r), (t >>>= 0), !n)) {
								var f = Math.pow(2, 8 * e - 1)
								j(this, r, t, e, f - 1, -f)
							}
							var i = 0,
								u = 1,
								o = 0
							for (this[t] = 255 & r; ++i < e && (u *= 256); ) r < 0 && 0 === o && 0 !== this[t + i - 1] && (o = 1), (this[t + i] = (((r / u) >> 0) - o) & 255)
							return t + e
						}),
						(o.prototype.writeIntBE = function (r, t, e, n) {
							if (((r = +r), (t >>>= 0), !n)) {
								var f = Math.pow(2, 8 * e - 1)
								j(this, r, t, e, f - 1, -f)
							}
							var i = e - 1,
								u = 1,
								o = 0
							for (this[t + i] = 255 & r; --i >= 0 && (u *= 256); ) r < 0 && 0 === o && 0 !== this[t + i + 1] && (o = 1), (this[t + i] = (((r / u) >> 0) - o) & 255)
							return t + e
						}),
						(o.prototype.writeInt8 = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 1, 127, -128), r < 0 && (r = 255 + r + 1), (this[t] = 255 & r), t + 1
						}),
						(o.prototype.writeInt16LE = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 2, 32767, -32768), (this[t] = 255 & r), (this[t + 1] = r >>> 8), t + 2
						}),
						(o.prototype.writeInt16BE = function (r, t, e) {
							return (r = +r), (t >>>= 0), e || j(this, r, t, 2, 32767, -32768), (this[t] = r >>> 8), (this[t + 1] = 255 & r), t + 2
						}),
						(o.prototype.writeInt32LE = function (r, t, e) {
							return (
								(r = +r),
								(t >>>= 0),
								e || j(this, r, t, 4, 2147483647, -2147483648),
								(this[t] = 255 & r),
								(this[t + 1] = r >>> 8),
								(this[t + 2] = r >>> 16),
								(this[t + 3] = r >>> 24),
								t + 4
							)
						}),
						(o.prototype.writeInt32BE = function (r, t, e) {
							return (
								(r = +r),
								(t >>>= 0),
								e || j(this, r, t, 4, 2147483647, -2147483648),
								r < 0 && (r = 4294967295 + r + 1),
								(this[t] = r >>> 24),
								(this[t + 1] = r >>> 16),
								(this[t + 2] = r >>> 8),
								(this[t + 3] = 255 & r),
								t + 4
							)
						}),
						(o.prototype.writeFloatLE = function (r, t, e) {
							return L(this, r, t, !0, e)
						}),
						(o.prototype.writeFloatBE = function (r, t, e) {
							return L(this, r, t, !1, e)
						}),
						(o.prototype.writeDoubleLE = function (r, t, e) {
							return S(this, r, t, !0, e)
						}),
						(o.prototype.writeDoubleBE = function (r, t, e) {
							return S(this, r, t, !1, e)
						}),
						(o.prototype.copy = function (r, t, e, n) {
							if (!o.isBuffer(r)) throw new TypeError("argument should be a Buffer")
							if ((e || (e = 0), n || 0 === n || (n = this.length), t >= r.length && (t = r.length), t || (t = 0), n > 0 && n < e && (n = e), n === e)) return 0
							if (0 === r.length || 0 === this.length) return 0
							if (t < 0) throw new RangeError("targetStart out of bounds")
							if (e < 0 || e >= this.length) throw new RangeError("Index out of range")
							if (n < 0) throw new RangeError("sourceEnd out of bounds")
							n > this.length && (n = this.length), r.length - t < n - e && (n = r.length - t + e)
							var f = n - e
							if (this === r && "function" == typeof Uint8Array.prototype.copyWithin) this.copyWithin(t, e, n)
							else if (this === r && e < t && t < n) for (var i = f - 1; i >= 0; --i) r[i + t] = this[i + e]
							else Uint8Array.prototype.set.call(r, this.subarray(e, n), t)
							return f
						}),
						(o.prototype.fill = function (r, t, e, n) {
							if ("string" == typeof r) {
								if (("string" == typeof t ? ((n = t), (t = 0), (e = this.length)) : "string" == typeof e && ((n = e), (e = this.length)), void 0 !== n && "string" != typeof n))
									throw new TypeError("encoding must be a string")
								if ("string" == typeof n && !o.isEncoding(n)) throw new TypeError("Unknown encoding: " + n)
								if (1 === r.length) {
									var f = r.charCodeAt(0)
									;(("utf8" === n && f < 128) || "latin1" === n) && (r = f)
								}
							} else "number" == typeof r && (r &= 255)
							if (t < 0 || this.length < t || this.length < e) throw new RangeError("Out of range index")
							if (e <= t) return this
							var i
							if (((t >>>= 0), (e = void 0 === e ? this.length : e >>> 0), r || (r = 0), "number" == typeof r)) for (i = t; i < e; ++i) this[i] = r
							else {
								var u = o.isBuffer(r) ? r : o.from(r, n),
									s = u.length
								if (0 === s) throw new TypeError('The value "' + r + '" is invalid for argument "value"')
								for (i = 0; i < e - t; ++i) this[i + t] = u[i % s]
							}
							return this
						})
					var I = /[^+\/0-9A-Za-z-_]/g
					function N(r) {
						return r < 16 ? "0" + r.toString(16) : r.toString(16)
					}
					function D(r, t) {
						var e
						t = t || 1 / 0
						for (var n = r.length, f = null, i = [], u = 0; u < n; ++u) {
							if ((e = r.charCodeAt(u)) > 55295 && e < 57344) {
								if (!f) {
									if (e > 56319) {
										;(t -= 3) > -1 && i.push(239, 191, 189)
										continue
									}
									if (u + 1 === n) {
										;(t -= 3) > -1 && i.push(239, 191, 189)
										continue
									}
									f = e
									continue
								}
								if (e < 56320) {
									;(t -= 3) > -1 && i.push(239, 191, 189), (f = e)
									continue
								}
								e = 65536 + (((f - 55296) << 10) | (e - 56320))
							} else f && (t -= 3) > -1 && i.push(239, 191, 189)
							if (((f = null), e < 128)) {
								if ((t -= 1) < 0) break
								i.push(e)
							} else if (e < 2048) {
								if ((t -= 2) < 0) break
								i.push((e >> 6) | 192, (63 & e) | 128)
							} else if (e < 65536) {
								if ((t -= 3) < 0) break
								i.push((e >> 12) | 224, ((e >> 6) & 63) | 128, (63 & e) | 128)
							} else {
								if (!(e < 1114112)) throw new Error("Invalid code point")
								if ((t -= 4) < 0) break
								i.push((e >> 18) | 240, ((e >> 12) & 63) | 128, ((e >> 6) & 63) | 128, (63 & e) | 128)
							}
						}
						return i
					}
					function q(r) {
						return n.toByteArray(
							(function (r) {
								if ((r = (r = r.split("=")[0]).trim().replace(I, "")).length < 2) return ""
								for (; r.length % 4 != 0; ) r += "="
								return r
							})(r)
						)
					}
					function F(r, t, e, n) {
						for (var f = 0; f < n && !(f + e >= t.length || f >= r.length); ++f) t[f + e] = r[f]
						return f
					}
					function Y(r, t) {
						return r instanceof t || (null != r && null != r.constructor && null != r.constructor.name && r.constructor.name === t.name)
					}
					function P(r) {
						return r != r
					}
				},
				{ "base64-js": 1, ieee754: 3 }
			],
			3: [
				function (r, t, e) {
					;(e.read = function (r, t, e, n, f) {
						var i,
							u,
							o = 8 * f - n - 1,
							s = (1 << o) - 1,
							a = s >> 1,
							c = -7,
							h = e ? f - 1 : 0,
							l = e ? -1 : 1,
							p = r[t + h]
						for (h += l, i = p & ((1 << -c) - 1), p >>= -c, c += o; c > 0; i = 256 * i + r[t + h], h += l, c -= 8);
						for (u = i & ((1 << -c) - 1), i >>= -c, c += n; c > 0; u = 256 * u + r[t + h], h += l, c -= 8);
						if (0 === i) i = 1 - a
						else {
							if (i === s) return u ? NaN : (1 / 0) * (p ? -1 : 1)
							;(u += Math.pow(2, n)), (i -= a)
						}
						return (p ? -1 : 1) * u * Math.pow(2, i - n)
					}),
						(e.write = function (r, t, e, n, f, i) {
							var u,
								o,
								s,
								a = 8 * i - f - 1,
								c = (1 << a) - 1,
								h = c >> 1,
								l = 23 === f ? Math.pow(2, -24) - Math.pow(2, -77) : 0,
								p = n ? 0 : i - 1,
								y = n ? 1 : -1,
								v = t < 0 || (0 === t && 1 / t < 0) ? 1 : 0
							for (
								t = Math.abs(t),
									isNaN(t) || t === 1 / 0
										? ((o = isNaN(t) ? 1 : 0), (u = c))
										: ((u = Math.floor(Math.log(t) / Math.LN2)),
											t * (s = Math.pow(2, -u)) < 1 && (u--, (s *= 2)),
											(t += u + h >= 1 ? l / s : l * Math.pow(2, 1 - h)) * s >= 2 && (u++, (s /= 2)),
											u + h >= c ? ((o = 0), (u = c)) : u + h >= 1 ? ((o = (t * s - 1) * Math.pow(2, f)), (u += h)) : ((o = t * Math.pow(2, h - 1) * Math.pow(2, f)), (u = 0)));
								f >= 8;
								r[e + p] = 255 & o, p += y, o /= 256, f -= 8
							);
							for (u = (u << f) | o, a += f; a > 0; r[e + p] = 255 & u, p += y, u /= 256, a -= 8);
							r[e + p - y] |= 128 * v
						})
				},
				{}
			],
			4: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc1").default
				},
				{ "./es6/crc1": 15 }
			],
			5: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc16").default
				},
				{ "./es6/crc16": 16 }
			],
			6: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc16ccitt").default
				},
				{ "./es6/crc16ccitt": 17 }
			],
			7: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc16kermit").default
				},
				{ "./es6/crc16kermit": 18 }
			],
			8: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc16modbus").default
				},
				{ "./es6/crc16modbus": 19 }
			],
			9: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc16xmodem").default
				},
				{ "./es6/crc16xmodem": 20 }
			],
			10: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc24").default
				},
				{ "./es6/crc24": 21 }
			],
			11: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc32").default
				},
				{ "./es6/crc32": 22 }
			],
			12: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc8").default
				},
				{ "./es6/crc8": 23 }
			],
			13: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crc81wire").default
				},
				{ "./es6/crc81wire": 24 }
			],
			14: [
				function (r, t, e) {
					"use strict"
					t.exports = r("./es6/crcjam").default
				},
				{ "./es6/crcjam": 25 }
			],
			15: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = i(r("./create_buffer"))
					function i(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var u = (0, i(r("./define_crc")).default)("crc1", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = ~~t, i = 0, u = 0; u < r.length; u++) {
							i += r[u]
						}
						return (e += i % 256) % 256
					})
					e.default = u
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			16: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 49345, 49537, 320, 49921, 960, 640, 49729, 50689, 1728, 1920, 51009, 1280, 50625, 50305, 1088, 52225, 3264, 3456, 52545, 3840, 53185, 52865, 3648, 2560, 51905, 52097, 2880,
						51457, 2496, 2176, 51265, 55297, 6336, 6528, 55617, 6912, 56257, 55937, 6720, 7680, 57025, 57217, 8e3, 56577, 7616, 7296, 56385, 5120, 54465, 54657, 5440, 55041, 6080, 5760,
						54849, 53761, 4800, 4992, 54081, 4352, 53697, 53377, 4160, 61441, 12480, 12672, 61761, 13056, 62401, 62081, 12864, 13824, 63169, 63361, 14144, 62721, 13760, 13440, 62529,
						15360, 64705, 64897, 15680, 65281, 16320, 16e3, 65089, 64001, 15040, 15232, 64321, 14592, 63937, 63617, 14400, 10240, 59585, 59777, 10560, 60161, 11200, 10880, 59969, 60929,
						11968, 12160, 61249, 11520, 60865, 60545, 11328, 58369, 9408, 9600, 58689, 9984, 59329, 59009, 9792, 8704, 58049, 58241, 9024, 57601, 8640, 8320, 57409, 40961, 24768, 24960,
						41281, 25344, 41921, 41601, 25152, 26112, 42689, 42881, 26432, 42241, 26048, 25728, 42049, 27648, 44225, 44417, 27968, 44801, 28608, 28288, 44609, 43521, 27328, 27520, 43841,
						26880, 43457, 43137, 26688, 30720, 47297, 47489, 31040, 47873, 31680, 31360, 47681, 48641, 32448, 32640, 48961, 32e3, 48577, 48257, 31808, 46081, 29888, 30080, 46401, 30464,
						47041, 46721, 30272, 29184, 45761, 45953, 29504, 45313, 29120, 28800, 45121, 20480, 37057, 37249, 20800, 37633, 21440, 21120, 37441, 38401, 22208, 22400, 38721, 21760, 38337,
						38017, 21568, 39937, 23744, 23936, 40257, 24320, 40897, 40577, 24128, 23040, 39617, 39809, 23360, 39169, 22976, 22656, 38977, 34817, 18624, 18816, 35137, 19200, 35777, 35457,
						19008, 19968, 36545, 36737, 20288, 36097, 19904, 19584, 35905, 17408, 33985, 34177, 17728, 34561, 18368, 18048, 34369, 33281, 17088, 17280, 33601, 16640, 33217, 32897, 16448
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("crc-16", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = ~~t, i = 0; i < r.length; i++) {
							var u = r[i]
							e = 65535 & (o[255 & (e ^ u)] ^ (e >> 8))
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			17: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 4129, 8258, 12387, 16516, 20645, 24774, 28903, 33032, 37161, 41290, 45419, 49548, 53677, 57806, 61935, 4657, 528, 12915, 8786, 21173, 17044, 29431, 25302, 37689, 33560,
						45947, 41818, 54205, 50076, 62463, 58334, 9314, 13379, 1056, 5121, 25830, 29895, 17572, 21637, 42346, 46411, 34088, 38153, 58862, 62927, 50604, 54669, 13907, 9842, 5649, 1584,
						30423, 26358, 22165, 18100, 46939, 42874, 38681, 34616, 63455, 59390, 55197, 51132, 18628, 22757, 26758, 30887, 2112, 6241, 10242, 14371, 51660, 55789, 59790, 63919, 35144,
						39273, 43274, 47403, 23285, 19156, 31415, 27286, 6769, 2640, 14899, 10770, 56317, 52188, 64447, 60318, 39801, 35672, 47931, 43802, 27814, 31879, 19684, 23749, 11298, 15363,
						3168, 7233, 60846, 64911, 52716, 56781, 44330, 48395, 36200, 40265, 32407, 28342, 24277, 20212, 15891, 11826, 7761, 3696, 65439, 61374, 57309, 53244, 48923, 44858, 40793,
						36728, 37256, 33193, 45514, 41451, 53516, 49453, 61774, 57711, 4224, 161, 12482, 8419, 20484, 16421, 28742, 24679, 33721, 37784, 41979, 46042, 49981, 54044, 58239, 62302, 689,
						4752, 8947, 13010, 16949, 21012, 25207, 29270, 46570, 42443, 38312, 34185, 62830, 58703, 54572, 50445, 13538, 9411, 5280, 1153, 29798, 25671, 21540, 17413, 42971, 47098, 34713,
						38840, 59231, 63358, 50973, 55100, 9939, 14066, 1681, 5808, 26199, 30326, 17941, 22068, 55628, 51565, 63758, 59695, 39368, 35305, 47498, 43435, 22596, 18533, 30726, 26663,
						6336, 2273, 14466, 10403, 52093, 56156, 60223, 64286, 35833, 39896, 43963, 48026, 19061, 23124, 27191, 31254, 2801, 6864, 10931, 14994, 64814, 60687, 56684, 52557, 48554,
						44427, 40424, 36297, 31782, 27655, 23652, 19525, 15522, 11395, 7392, 3265, 61215, 65342, 53085, 57212, 44955, 49082, 36825, 40952, 28183, 32310, 20053, 24180, 11923, 16050,
						3793, 7920
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("ccitt", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = void 0 !== t ? ~~t : 65535, i = 0; i < r.length; i++) {
							var u = r[i]
							e = 65535 & (o[255 & ((e >> 8) ^ u)] ^ (e << 8))
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			18: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 4489, 8978, 12955, 17956, 22445, 25910, 29887, 35912, 40385, 44890, 48851, 51820, 56293, 59774, 63735, 4225, 264, 13203, 8730, 22181, 18220, 30135, 25662, 40137, 36160,
						49115, 44626, 56045, 52068, 63999, 59510, 8450, 12427, 528, 5017, 26406, 30383, 17460, 21949, 44362, 48323, 36440, 40913, 60270, 64231, 51324, 55797, 12675, 8202, 4753, 792,
						30631, 26158, 21685, 17724, 48587, 44098, 40665, 36688, 64495, 60006, 55549, 51572, 16900, 21389, 24854, 28831, 1056, 5545, 10034, 14011, 52812, 57285, 60766, 64727, 34920,
						39393, 43898, 47859, 21125, 17164, 29079, 24606, 5281, 1320, 14259, 9786, 57037, 53060, 64991, 60502, 39145, 35168, 48123, 43634, 25350, 29327, 16404, 20893, 9506, 13483, 1584,
						6073, 61262, 65223, 52316, 56789, 43370, 47331, 35448, 39921, 29575, 25102, 20629, 16668, 13731, 9258, 5809, 1848, 65487, 60998, 56541, 52564, 47595, 43106, 39673, 35696,
						33800, 38273, 42778, 46739, 49708, 54181, 57662, 61623, 2112, 6601, 11090, 15067, 20068, 24557, 28022, 31999, 38025, 34048, 47003, 42514, 53933, 49956, 61887, 57398, 6337,
						2376, 15315, 10842, 24293, 20332, 32247, 27774, 42250, 46211, 34328, 38801, 58158, 62119, 49212, 53685, 10562, 14539, 2640, 7129, 28518, 32495, 19572, 24061, 46475, 41986,
						38553, 34576, 62383, 57894, 53437, 49460, 14787, 10314, 6865, 2904, 32743, 28270, 23797, 19836, 50700, 55173, 58654, 62615, 32808, 37281, 41786, 45747, 19012, 23501, 26966,
						30943, 3168, 7657, 12146, 16123, 54925, 50948, 62879, 58390, 37033, 33056, 46011, 41522, 23237, 19276, 31191, 26718, 7393, 3432, 16371, 11898, 59150, 63111, 50204, 54677,
						41258, 45219, 33336, 37809, 27462, 31439, 18516, 23005, 11618, 15595, 3696, 8185, 63375, 58886, 54429, 50452, 45483, 40994, 37561, 33584, 31687, 27214, 22741, 18780, 15843,
						11370, 7921, 3960
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("kermit", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = void 0 !== t ? ~~t : 0, i = 0; i < r.length; i++) {
							var u = r[i]
							e = 65535 & (o[255 & (e ^ u)] ^ (e >> 8))
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			19: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 49345, 49537, 320, 49921, 960, 640, 49729, 50689, 1728, 1920, 51009, 1280, 50625, 50305, 1088, 52225, 3264, 3456, 52545, 3840, 53185, 52865, 3648, 2560, 51905, 52097, 2880,
						51457, 2496, 2176, 51265, 55297, 6336, 6528, 55617, 6912, 56257, 55937, 6720, 7680, 57025, 57217, 8e3, 56577, 7616, 7296, 56385, 5120, 54465, 54657, 5440, 55041, 6080, 5760,
						54849, 53761, 4800, 4992, 54081, 4352, 53697, 53377, 4160, 61441, 12480, 12672, 61761, 13056, 62401, 62081, 12864, 13824, 63169, 63361, 14144, 62721, 13760, 13440, 62529,
						15360, 64705, 64897, 15680, 65281, 16320, 16e3, 65089, 64001, 15040, 15232, 64321, 14592, 63937, 63617, 14400, 10240, 59585, 59777, 10560, 60161, 11200, 10880, 59969, 60929,
						11968, 12160, 61249, 11520, 60865, 60545, 11328, 58369, 9408, 9600, 58689, 9984, 59329, 59009, 9792, 8704, 58049, 58241, 9024, 57601, 8640, 8320, 57409, 40961, 24768, 24960,
						41281, 25344, 41921, 41601, 25152, 26112, 42689, 42881, 26432, 42241, 26048, 25728, 42049, 27648, 44225, 44417, 27968, 44801, 28608, 28288, 44609, 43521, 27328, 27520, 43841,
						26880, 43457, 43137, 26688, 30720, 47297, 47489, 31040, 47873, 31680, 31360, 47681, 48641, 32448, 32640, 48961, 32e3, 48577, 48257, 31808, 46081, 29888, 30080, 46401, 30464,
						47041, 46721, 30272, 29184, 45761, 45953, 29504, 45313, 29120, 28800, 45121, 20480, 37057, 37249, 20800, 37633, 21440, 21120, 37441, 38401, 22208, 22400, 38721, 21760, 38337,
						38017, 21568, 39937, 23744, 23936, 40257, 24320, 40897, 40577, 24128, 23040, 39617, 39809, 23360, 39169, 22976, 22656, 38977, 34817, 18624, 18816, 35137, 19200, 35777, 35457,
						19008, 19968, 36545, 36737, 20288, 36097, 19904, 19584, 35905, 17408, 33985, 34177, 17728, 34561, 18368, 18048, 34369, 33281, 17088, 17280, 33601, 16640, 33217, 32897, 16448
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("crc-16-modbus", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = void 0 !== t ? ~~t : 65535, i = 0; i < r.length; i++) {
							var u = r[i]
							e = 65535 & (o[255 & (e ^ u)] ^ (e >> 8))
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			20: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = i(r("./create_buffer"))
					function i(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var u = (0, i(r("./define_crc")).default)("xmodem", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = void 0 !== t ? ~~t : 0, i = 0; i < r.length; i++) {
							var u = (e >>> 8) & 255
							;(u ^= 255 & r[i]), (e = (e << 8) & 65535), (e ^= u ^= u >>> 4), (e ^= u = (u << 5) & 65535), (e ^= u = (u << 7) & 65535)
						}
						return e
					})
					e.default = u
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			21: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 8801531, 9098509, 825846, 9692897, 1419802, 1651692, 10452759, 10584377, 2608578, 2839604, 11344079, 3303384, 11807523, 12104405, 4128302, 12930697, 4391538, 5217156,
						13227903, 5679208, 13690003, 14450021, 5910942, 6606768, 14844747, 15604413, 6837830, 16197969, 7431594, 8256604, 16494759, 840169, 9084178, 8783076, 18463, 10434312, 1670131,
						1434117, 9678590, 11358416, 2825259, 2590173, 10602790, 4109873, 12122826, 11821884, 3289031, 13213536, 5231515, 4409965, 12912278, 5929345, 14431610, 13675660, 5693559,
						6823513, 15618722, 14863188, 6588335, 16513208, 8238147, 7417269, 16212302, 1680338, 10481449, 9664223, 1391140, 9061683, 788936, 36926, 8838341, 12067563, 4091408, 3340262,
						11844381, 2868234, 11372785, 10555655, 2579964, 14478683, 5939616, 5650518, 13661357, 5180346, 13190977, 12967607, 4428364, 8219746, 16457881, 16234863, 7468436, 15633027,
						6866552, 6578062, 14816117, 1405499, 9649856, 10463030, 1698765, 8819930, 55329, 803287, 9047340, 11858690, 3325945, 4072975, 12086004, 2561507, 10574104, 11387118, 2853909,
						13647026, 5664841, 5958079, 14460228, 4446803, 12949160, 13176670, 5194661, 7454091, 16249200, 16476294, 8201341, 14834538, 6559633, 6852199, 15647388, 3360676, 11864927,
						12161705, 4185682, 10527045, 2551230, 2782280, 11286707, 9619101, 1346150, 1577872, 10379115, 73852, 8875143, 9172337, 899466, 16124205, 7357910, 8182816, 16421083, 6680524,
						14918455, 15678145, 6911546, 5736468, 13747439, 14507289, 5968354, 12873461, 4334094, 5159928, 13170435, 4167245, 12180150, 11879232, 3346363, 11301036, 2767959, 2532769,
						10545498, 10360692, 1596303, 1360505, 9604738, 913813, 9157998, 8856728, 92259, 16439492, 8164415, 7343561, 16138546, 6897189, 15692510, 14936872, 6662099, 5986813, 14488838,
						13733104, 5750795, 13156124, 5174247, 4352529, 12855018, 2810998, 11315341, 10498427, 2522496, 12124823, 4148844, 3397530, 11901793, 9135439, 862644, 110658, 8912057, 1606574,
						10407765, 9590435, 1317464, 15706879, 6940164, 6651890, 14889737, 8145950, 16384229, 16161043, 7394792, 5123014, 13133629, 12910283, 4370992, 14535975, 5997020, 5707818,
						13718737, 2504095, 10516836, 11329682, 2796649, 11916158, 3383173, 4130419, 12143240, 8893606, 129117, 876971, 9121104, 1331783, 9576124, 10389322, 1625009, 14908182, 6633453,
						6925851, 15721184, 7380471, 16175372, 16402682, 8127489, 4389423, 12891860, 13119266, 5137369, 13704398, 5722165, 6015427, 14517560
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("crc-24", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = void 0 !== t ? ~~t : 11994318, i = 0; i < r.length; i++) {
							var u = r[i]
							e = 16777215 & (o[255 & ((e >> 16) ^ u)] ^ (e << 8))
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			22: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 1996959894, 3993919788, 2567524794, 124634137, 1886057615, 3915621685, 2657392035, 249268274, 2044508324, 3772115230, 2547177864, 162941995, 2125561021, 3887607047,
						2428444049, 498536548, 1789927666, 4089016648, 2227061214, 450548861, 1843258603, 4107580753, 2211677639, 325883990, 1684777152, 4251122042, 2321926636, 335633487, 1661365465,
						4195302755, 2366115317, 997073096, 1281953886, 3579855332, 2724688242, 1006888145, 1258607687, 3524101629, 2768942443, 901097722, 1119000684, 3686517206, 2898065728, 853044451,
						1172266101, 3705015759, 2882616665, 651767980, 1373503546, 3369554304, 3218104598, 565507253, 1454621731, 3485111705, 3099436303, 671266974, 1594198024, 3322730930, 2970347812,
						795835527, 1483230225, 3244367275, 3060149565, 1994146192, 31158534, 2563907772, 4023717930, 1907459465, 112637215, 2680153253, 3904427059, 2013776290, 251722036, 2517215374,
						3775830040, 2137656763, 141376813, 2439277719, 3865271297, 1802195444, 476864866, 2238001368, 4066508878, 1812370925, 453092731, 2181625025, 4111451223, 1706088902, 314042704,
						2344532202, 4240017532, 1658658271, 366619977, 2362670323, 4224994405, 1303535960, 984961486, 2747007092, 3569037538, 1256170817, 1037604311, 2765210733, 3554079995,
						1131014506, 879679996, 2909243462, 3663771856, 1141124467, 855842277, 2852801631, 3708648649, 1342533948, 654459306, 3188396048, 3373015174, 1466479909, 544179635, 3110523913,
						3462522015, 1591671054, 702138776, 2966460450, 3352799412, 1504918807, 783551873, 3082640443, 3233442989, 3988292384, 2596254646, 62317068, 1957810842, 3939845945, 2647816111,
						81470997, 1943803523, 3814918930, 2489596804, 225274430, 2053790376, 3826175755, 2466906013, 167816743, 2097651377, 4027552580, 2265490386, 503444072, 1762050814, 4150417245,
						2154129355, 426522225, 1852507879, 4275313526, 2312317920, 282753626, 1742555852, 4189708143, 2394877945, 397917763, 1622183637, 3604390888, 2714866558, 953729732, 1340076626,
						3518719985, 2797360999, 1068828381, 1219638859, 3624741850, 2936675148, 906185462, 1090812512, 3747672003, 2825379669, 829329135, 1181335161, 3412177804, 3160834842, 628085408,
						1382605366, 3423369109, 3138078467, 570562233, 1426400815, 3317316542, 2998733608, 733239954, 1555261956, 3268935591, 3050360625, 752459403, 1541320221, 2607071920, 3965973030,
						1969922972, 40735498, 2617837225, 3943577151, 1913087877, 83908371, 2512341634, 3803740692, 2075208622, 213261112, 2463272603, 3855990285, 2094854071, 198958881, 2262029012,
						4057260610, 1759359992, 534414190, 2176718541, 4139329115, 1873836001, 414664567, 2282248934, 4279200368, 1711684554, 285281116, 2405801727, 4167216745, 1634467795, 376229701,
						2685067896, 3608007406, 1308918612, 956543938, 2808555105, 3495958263, 1231636301, 1047427035, 2932959818, 3654703836, 1088359270, 936918e3, 2847714899, 3736837829, 1202900863,
						817233897, 3183342108, 3401237130, 1404277552, 615818150, 3134207493, 3453421203, 1423857449, 601450431, 3009837614, 3294710456, 1567103746, 711928724, 3020668471, 3272380065,
						1510334235, 755167117
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("crc-32", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = 0 === t ? 0 : -1 ^ ~~t, i = 0; i < r.length; i++) {
							var u = r[i]
							e = o[255 & (e ^ u)] ^ (e >>> 8)
						}
						return -1 ^ e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			23: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 7, 14, 9, 28, 27, 18, 21, 56, 63, 54, 49, 36, 35, 42, 45, 112, 119, 126, 121, 108, 107, 98, 101, 72, 79, 70, 65, 84, 83, 90, 93, 224, 231, 238, 233, 252, 251, 242, 245, 216,
						223, 214, 209, 196, 195, 202, 205, 144, 151, 158, 153, 140, 139, 130, 133, 168, 175, 166, 161, 180, 179, 186, 189, 199, 192, 201, 206, 219, 220, 213, 210, 255, 248, 241, 246,
						227, 228, 237, 234, 183, 176, 185, 190, 171, 172, 165, 162, 143, 136, 129, 134, 147, 148, 157, 154, 39, 32, 41, 46, 59, 60, 53, 50, 31, 24, 17, 22, 3, 4, 13, 10, 87, 80, 89,
						94, 75, 76, 69, 66, 111, 104, 97, 102, 115, 116, 125, 122, 137, 142, 135, 128, 149, 146, 155, 156, 177, 182, 191, 184, 173, 170, 163, 164, 249, 254, 247, 240, 229, 226, 235,
						236, 193, 198, 207, 200, 221, 218, 211, 212, 105, 110, 103, 96, 117, 114, 123, 124, 81, 86, 95, 88, 77, 74, 67, 68, 25, 30, 23, 16, 5, 2, 11, 12, 33, 38, 47, 40, 61, 58, 51,
						52, 78, 73, 64, 71, 82, 85, 92, 91, 118, 113, 120, 127, 106, 109, 100, 99, 62, 57, 48, 55, 34, 37, 44, 43, 6, 1, 8, 15, 26, 29, 20, 19, 174, 169, 160, 167, 178, 181, 188, 187,
						150, 145, 152, 159, 138, 141, 132, 131, 222, 217, 208, 215, 194, 197, 204, 203, 230, 225, 232, 239, 250, 253, 244, 243
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("crc-8", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = ~~t, i = 0; i < r.length; i++) {
							var u = r[i]
							e = 255 & o[255 & (e ^ u)]
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			24: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 94, 188, 226, 97, 63, 221, 131, 194, 156, 126, 32, 163, 253, 31, 65, 157, 195, 33, 127, 252, 162, 64, 30, 95, 1, 227, 189, 62, 96, 130, 220, 35, 125, 159, 193, 66, 28, 254,
						160, 225, 191, 93, 3, 128, 222, 60, 98, 190, 224, 2, 92, 223, 129, 99, 61, 124, 34, 192, 158, 29, 67, 161, 255, 70, 24, 250, 164, 39, 121, 155, 197, 132, 218, 56, 102, 229,
						187, 89, 7, 219, 133, 103, 57, 186, 228, 6, 88, 25, 71, 165, 251, 120, 38, 196, 154, 101, 59, 217, 135, 4, 90, 184, 230, 167, 249, 27, 69, 198, 152, 122, 36, 248, 166, 68, 26,
						153, 199, 37, 123, 58, 100, 134, 216, 91, 5, 231, 185, 140, 210, 48, 110, 237, 179, 81, 15, 78, 16, 242, 172, 47, 113, 147, 205, 17, 79, 173, 243, 112, 46, 204, 146, 211, 141,
						111, 49, 178, 236, 14, 80, 175, 241, 19, 77, 206, 144, 114, 44, 109, 51, 209, 143, 12, 82, 176, 238, 50, 108, 142, 208, 83, 13, 239, 177, 240, 174, 76, 18, 145, 207, 45, 115,
						202, 148, 118, 40, 171, 245, 23, 73, 8, 86, 180, 234, 105, 55, 213, 139, 87, 9, 235, 181, 54, 104, 138, 212, 149, 203, 41, 119, 244, 170, 72, 22, 233, 183, 85, 11, 136, 214,
						52, 106, 43, 117, 151, 201, 74, 20, 246, 168, 116, 42, 200, 150, 21, 75, 169, 247, 182, 232, 10, 84, 215, 137, 107, 53
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("dallas-1-wire", function (r, t) {
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = ~~t, i = 0; i < r.length; i++) {
							var u = r[i]
							e = 255 & o[255 & (e ^ u)]
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			25: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f = u(r("./create_buffer")),
						i = u(r("./define_crc"))
					function u(r) {
						return r && r.__esModule ? r : { default: r }
					}
					var o = [
						0, 1996959894, 3993919788, 2567524794, 124634137, 1886057615, 3915621685, 2657392035, 249268274, 2044508324, 3772115230, 2547177864, 162941995, 2125561021, 3887607047,
						2428444049, 498536548, 1789927666, 4089016648, 2227061214, 450548861, 1843258603, 4107580753, 2211677639, 325883990, 1684777152, 4251122042, 2321926636, 335633487, 1661365465,
						4195302755, 2366115317, 997073096, 1281953886, 3579855332, 2724688242, 1006888145, 1258607687, 3524101629, 2768942443, 901097722, 1119000684, 3686517206, 2898065728, 853044451,
						1172266101, 3705015759, 2882616665, 651767980, 1373503546, 3369554304, 3218104598, 565507253, 1454621731, 3485111705, 3099436303, 671266974, 1594198024, 3322730930, 2970347812,
						795835527, 1483230225, 3244367275, 3060149565, 1994146192, 31158534, 2563907772, 4023717930, 1907459465, 112637215, 2680153253, 3904427059, 2013776290, 251722036, 2517215374,
						3775830040, 2137656763, 141376813, 2439277719, 3865271297, 1802195444, 476864866, 2238001368, 4066508878, 1812370925, 453092731, 2181625025, 4111451223, 1706088902, 314042704,
						2344532202, 4240017532, 1658658271, 366619977, 2362670323, 4224994405, 1303535960, 984961486, 2747007092, 3569037538, 1256170817, 1037604311, 2765210733, 3554079995,
						1131014506, 879679996, 2909243462, 3663771856, 1141124467, 855842277, 2852801631, 3708648649, 1342533948, 654459306, 3188396048, 3373015174, 1466479909, 544179635, 3110523913,
						3462522015, 1591671054, 702138776, 2966460450, 3352799412, 1504918807, 783551873, 3082640443, 3233442989, 3988292384, 2596254646, 62317068, 1957810842, 3939845945, 2647816111,
						81470997, 1943803523, 3814918930, 2489596804, 225274430, 2053790376, 3826175755, 2466906013, 167816743, 2097651377, 4027552580, 2265490386, 503444072, 1762050814, 4150417245,
						2154129355, 426522225, 1852507879, 4275313526, 2312317920, 282753626, 1742555852, 4189708143, 2394877945, 397917763, 1622183637, 3604390888, 2714866558, 953729732, 1340076626,
						3518719985, 2797360999, 1068828381, 1219638859, 3624741850, 2936675148, 906185462, 1090812512, 3747672003, 2825379669, 829329135, 1181335161, 3412177804, 3160834842, 628085408,
						1382605366, 3423369109, 3138078467, 570562233, 1426400815, 3317316542, 2998733608, 733239954, 1555261956, 3268935591, 3050360625, 752459403, 1541320221, 2607071920, 3965973030,
						1969922972, 40735498, 2617837225, 3943577151, 1913087877, 83908371, 2512341634, 3803740692, 2075208622, 213261112, 2463272603, 3855990285, 2094854071, 198958881, 2262029012,
						4057260610, 1759359992, 534414190, 2176718541, 4139329115, 1873836001, 414664567, 2282248934, 4279200368, 1711684554, 285281116, 2405801727, 4167216745, 1634467795, 376229701,
						2685067896, 3608007406, 1308918612, 956543938, 2808555105, 3495958263, 1231636301, 1047427035, 2932959818, 3654703836, 1088359270, 936918e3, 2847714899, 3736837829, 1202900863,
						817233897, 3183342108, 3401237130, 1404277552, 615818150, 3134207493, 3453421203, 1423857449, 601450431, 3009837614, 3294710456, 1567103746, 711928724, 3020668471, 3272380065,
						1510334235, 755167117
					]
					"undefined" != typeof Int32Array && (o = new Int32Array(o))
					var s = (0, i.default)("jam", function (r) {
						var t = arguments.length > 1 && void 0 !== arguments[1] ? arguments[1] : -1
						n.Buffer.isBuffer(r) || (r = (0, f.default)(r))
						for (var e = 0 === t ? 0 : ~~t, i = 0; i < r.length; i++) {
							var u = r[i]
							e = o[255 & (e ^ u)] ^ (e >>> 8)
						}
						return e
					})
					e.default = s
				},
				{ "./create_buffer": 26, "./define_crc": 27, buffer: 2 }
			],
			26: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 })
					var n = r("buffer"),
						f =
							n.Buffer.from && n.Buffer.alloc && n.Buffer.allocUnsafe && n.Buffer.allocUnsafeSlow
								? n.Buffer.from
								: function (r) {
										return new n.Buffer(r)
									}
					e.default = f
				},
				{ buffer: 2 }
			],
			27: [
				function (r, t, e) {
					"use strict"
					Object.defineProperty(e, "__esModule", { value: !0 }),
						(e.default = function (r, t) {
							var e = function (r, e) {
								return t(r, e) >>> 0
							}
							return (e.signed = t), (e.unsigned = e), (e.model = r), e
						})
				},
				{}
			],
			28: [
				function (r, t, e) {
					"use strict"
					t.exports = {
						crc1: r("./crc1"),
						crc8: r("./crc8"),
						crc81wire: r("./crc8_1wire"),
						crc16: r("./crc16"),
						crc16ccitt: r("./crc16_ccitt"),
						crc16modbus: r("./crc16_modbus"),
						crc16xmodem: r("./crc16_xmodem"),
						crc16kermit: r("./crc16_kermit"),
						crc24: r("./crc24"),
						crc32: r("./crc32"),
						crcjam: r("./crcjam")
					}
				},
				{
					"./crc1": 4,
					"./crc16": 5,
					"./crc16_ccitt": 6,
					"./crc16_kermit": 7,
					"./crc16_modbus": 8,
					"./crc16_xmodem": 9,
					"./crc24": 10,
					"./crc32": 11,
					"./crc8": 12,
					"./crc8_1wire": 13,
					"./crcjam": 14
				}
			]
		},
		{},
		[28]
	)(28)
})
