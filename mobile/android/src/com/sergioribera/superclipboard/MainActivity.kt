package com.sergioribera.superclipboard

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import com.sergioribera.superclipboard.data.PreferencesManager
import com.sergioribera.superclipboard.services.UDPService
import com.sergioribera.superclipboard.ui.screens.Router
import com.sergioribera.superclipboard.ui.theme.AppTheme
import uniffi.mdns.MDnsDevice
import java.util.UUID

class MainActivity : ComponentActivity() {

    private lateinit var devicesManager: PreferencesManager

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.setProperty("uniffi.component.mdns.libraryOverride", "shared")
        devicesManager =
            PreferencesManager(application, "devices", listener = null)

        if (!devicesManager.containsKey("me")) {
            // generate new MDnsDevice
            val myDevice = MDnsDevice(UUID.randomUUID().toString(), android.os.Build.MODEL, "android")
            devicesManager.put(myDevice, "me")
        }

        val serviceIntent = Intent(this, UDPService::class.java)
        startService(serviceIntent)

        setContent {
            AppTheme {
                Router({})
            }
        }
    }

}