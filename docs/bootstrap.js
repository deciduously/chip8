/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/chip8_bg.wasm": function() {
/******/ 			return {
/******/ 				"./chip8_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_fbe0320f34c4cd31": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_instanceof_Window_fbe0320f34c4cd31"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_2b44f2a86e03665a": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_document_2b44f2a86e03665a"](p0i32);
/******/ 					},
/******/ 					"__wbg_setInterval_006c702c8f19b4ce": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_setInterval_006c702c8f19b4ce"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_body_08ba7a3043ff8e77": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_body_08ba7a3043ff8e77"](p0i32);
/******/ 					},
/******/ 					"__wbg_createAttribute_02f8d7590539fbad": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_createAttribute_02f8d7590539fbad"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_createElement_7cbe07ad3289abea": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_createElement_7cbe07ad3289abea"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_createTextNode_bbff6f9f6e6b38bf": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_createTextNode_bbff6f9f6e6b38bf"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getElementById_5bd6efc3d82494aa": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_getElementById_5bd6efc3d82494aa"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_bd2459c62d076bcd": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_bd2459c62d076bcd"](p0i32);
/******/ 					},
/******/ 					"__wbg_width_8225e9e48185d280": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_width_8225e9e48185d280"](p0i32);
/******/ 					},
/******/ 					"__wbg_setwidth_80b60efe20240a3e": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_setwidth_80b60efe20240a3e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_height_c55678b905b560e1": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_height_c55678b905b560e1"](p0i32);
/******/ 					},
/******/ 					"__wbg_setheight_5c308278bb4139ed": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_setheight_5c308278bb4139ed"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getContext_7f0328be9fe8c1ec": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_getContext_7f0328be9fe8c1ec"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setvalue_4f8d5f6832ca131d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_setvalue_4f8d5f6832ca131d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setAttributeNode_3940a7f2ecb93456": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_setAttributeNode_3940a7f2ecb93456"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_9783be44659339ea": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_error_9783be44659339ea"](p0i32);
/******/ 					},
/******/ 					"__wbg_log_2e875b1d2f6f87ac": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_log_2e875b1d2f6f87ac"](p0i32);
/******/ 					},
/******/ 					"__wbg_setonchange_c96927f1dfe22c6b": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_setonchange_c96927f1dfe22c6b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_CanvasRenderingContext2d_302c6fce2ddc6344": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_instanceof_CanvasRenderingContext2d_302c6fce2ddc6344"](p0i32);
/******/ 					},
/******/ 					"__wbg_setfillStyle_73b5e2cc68bb713a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_setfillStyle_73b5e2cc68bb713a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_beginPath_d5a5655e2cdb2690": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_beginPath_d5a5655e2cdb2690"](p0i32);
/******/ 					},
/******/ 					"__wbg_stroke_d68a036b0b76a1e2": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_stroke_d68a036b0b76a1e2"](p0i32);
/******/ 					},
/******/ 					"__wbg_clearRect_4cdcaefcbab3c61f": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_clearRect_4cdcaefcbab3c61f"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_fillRect_fc9267fcb85f10fd": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_fillRect_fc9267fcb85f10fd"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_scale_59eef7e0dbbdd0f4": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_scale_59eef7e0dbbdd0f4"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_appendChild_98dedaeac24501f2": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_appendChild_98dedaeac24501f2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlSelectElement_9e453923d9cacda8": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_instanceof_HtmlSelectElement_9e453923d9cacda8"](p0i32);
/******/ 					},
/******/ 					"__wbg_value_b431982196f2362c": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_value_b431982196f2362c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_ab183a630df3a257": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_call_ab183a630df3a257"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_ab5e899738c0eff4": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_newnoargs_ab5e899738c0eff4"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_77eca7b42660e1bb": function() {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_self_77eca7b42660e1bb"]();
/******/ 					},
/******/ 					"__wbg_window_51dac01569f1ba70": function() {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_window_51dac01569f1ba70"]();
/******/ 					},
/******/ 					"__wbg_globalThis_34bac2d08ebb9b58": function() {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_globalThis_34bac2d08ebb9b58"]();
/******/ 					},
/******/ 					"__wbg_global_1c436164a66c9c22": function() {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_global_1c436164a66c9c22"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_floor_65d9b37add803b26": function(p0f64) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_floor_65d9b37add803b26"](p0f64);
/******/ 					},
/******/ 					"__wbg_random_eb1fab8e1db2d9d1": function() {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_random_eb1fab8e1db2d9d1"]();
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper101": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_closure_wrapper101"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper103": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/chip8_bg.js"].exports["__wbindgen_closure_wrapper103"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/chip8_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/chip8_bg.wasm":"5e22f7da538cb3f5d12a"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });