/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(const_fn)]
#![feature(const_ptr_null)]
#![cfg(feature = "debugmozjs")]

#[macro_use]
extern crate js;
extern crate libc;

use js::jsapi::CompartmentOptions;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JSClass;
use js::jsapi::JSContext;
use js::jsapi::JSFunctionSpec;
use js::jsapi::JS_GetObjectPrototype;
use js::jsapi::JSNativeWrapper;
use js::jsapi::JS_NewGlobalObject;
use js::jsapi::JS_NewObjectWithUniqueType;
use js::jsapi::JSPROP_ENUMERATE;
use js::jsapi::JS_SetGCZeal;
use js::jsapi::OnNewGlobalHookOption;
use js::jsapi::Value;
use js::rust::{Runtime, SIMPLE_GLOBAL_CLASS, define_methods};
use std::ptr;

#[test]
fn rooting() {
    unsafe {
        let runtime = Runtime::new().unwrap();
        JS_SetGCZeal(runtime.rt(), 2, 1);

        let cx = runtime.cx();
        let h_option = OnNewGlobalHookOption::FireOnNewGlobalHook;
        let c_option = CompartmentOptions::default();

        rooted!(in(cx) let global = JS_NewGlobalObject(cx,
                                                       &SIMPLE_GLOBAL_CLASS,
                                                       ptr::null_mut(),
                                                       h_option,
                                                       &c_option));
        let _ac = JSAutoCompartment::new(cx, global.get());
        rooted!(in(cx) let prototype_proto = JS_GetObjectPrototype(cx, global.handle()));
        rooted!(in(cx) let proto = JS_NewObjectWithUniqueType(cx,
                                                              &CLASS as *const _,
                                                              prototype_proto.handle()));
        define_methods(cx, proto.handle(), METHODS).unwrap();
    }
}

unsafe extern "C" fn generic_method(_: *mut JSContext, _: u32, _: *mut Value) -> bool {
    true
}

const METHODS: &'static [JSFunctionSpec] = &[
    JSFunctionSpec {
        name: b"addEventListener\0" as *const u8 as *const libc::c_char,
        call: JSNativeWrapper { op: Some(generic_method), info: ptr::null() },
        nargs: 2,
        flags: JSPROP_ENUMERATE as u16,
        selfHostedName: 0 as *const libc::c_char
    },
    JSFunctionSpec {
        name: b"removeEventListener\0" as *const u8 as *const libc::c_char,
        call: JSNativeWrapper { op: Some(generic_method), info: ptr::null() },
        nargs: 2,
        flags: JSPROP_ENUMERATE as u16,
        selfHostedName: 0 as *const libc::c_char
    },
    JSFunctionSpec {
        name: b"dispatchEvent\0" as *const u8 as *const libc::c_char,
        call: JSNativeWrapper { op: Some(generic_method), info: ptr::null() },
        nargs: 1,
        flags: JSPROP_ENUMERATE as u16,
        selfHostedName: 0 as *const libc::c_char
    },
    JSFunctionSpec {
        name: ptr::null(),
        call: JSNativeWrapper { op: None, info: ptr::null() },
        nargs: 0,
        flags: 0,
        selfHostedName: ptr::null()
    }
];

static CLASS: JSClass = JSClass {
    name: b"EventTargetPrototype\0" as *const u8 as *const libc::c_char,
    flags: 0,
    cOps: 0 as *const _,
    reserved: [0 as *mut _; 3]
};
