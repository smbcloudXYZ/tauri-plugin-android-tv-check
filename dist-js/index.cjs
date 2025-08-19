'use strict';

var core = require('@tauri-apps/api/core');

async function check() {
    return await core.invoke("plugin:android-tv-check|check");
}

exports.check = check;
