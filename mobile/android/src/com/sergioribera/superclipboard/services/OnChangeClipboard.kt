package com.sergioribera.superclipboard.services

import android.app.Service
import android.content.ClipboardManager
import android.content.Intent
import android.os.IBinder

class OnChangeClipboard : Service() {

    private var clipboardManager: ClipboardManager? = null

    override fun onCreate() {
        super.onCreate()
        clipboardManager = (this.getSystemService(CLIPBOARD_SERVICE) as ClipboardManager)
        clipboardManager!!.addPrimaryClipChangedListener {
            onChangeClipboard()
        }
    }

    override fun onBind(intent: Intent): IBinder {
        // TODO: create multicast connection to send changes
        TODO("Return the communication channel to the service.")
    }

    fun onChangeClipboard() {
        val text = clipboardManager!!.primaryClip!!.getItemAt(0).text
        if (text != null) {
            // TODO: Send by multicast UDP to other devices
        }
    }
}