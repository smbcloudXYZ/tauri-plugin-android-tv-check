package com.plugin.androidTvCheck

import android.app.Activity
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class AndroidTvCheckPlugin(private val activity: Activity) : Plugin(activity) {
    private val implementation = Example()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun check(invoke: Invoke) {
        val TAG = "DeviceTypeRuntimeCheck"

        val isTelevision = false
        if (isTelevision) {
            Log.d(TAG, "Running on a TV Device")
            val ret = JSObject()
            ret.put("value", implementation.pong("default value :("))
            invoke.resolve(ret)
        } else {
            Log.d(TAG, "Running on a non-TV Device")
            val ret = JSObject()
            ret.put("value", implementation.pong("default value :("))
            invoke.resolve(ret)
        }
    }
}
