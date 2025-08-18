package com.plugin.androidTvCheck

import android.app.Activity
import android.content.pm.PackageManager
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class AndroidTvCheckPlugin(private val activity: Activity) : Plugin(activity) {
    @Command
    fun check(invoke: Invoke) {
        val TAG = "DeviceTypeRuntimeCheck"

        val isTelevision = activity.packageManager.hasSystemFeature(PackageManager.FEATURE_LEANBACK)

        Log.d(TAG, if (isTelevision) "Running on a TV Device" else "Running on a non-TV Device")

        val ret = JSObject()
        ret.put("is_android_tv", isTelevision)
        invoke.resolve(ret)
    }
}
