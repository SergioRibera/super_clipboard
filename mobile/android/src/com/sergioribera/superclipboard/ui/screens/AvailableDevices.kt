package com.sergioribera.superclipboard.ui.screens

import android.annotation.SuppressLint
import android.content.SharedPreferences
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.sergioribera.superclipboard.data.PreferencesManager
import com.sergioribera.superclipboard.ui.component.DeviceComponent
import uniffi.mdns.MDnsDevice

private lateinit var devicesManager: PreferencesManager
private var availableDevices: List<MDnsDevice> = emptyList()

@SuppressLint("UnusedMaterialScaffoldPaddingParameter")
@Composable
fun AvailableDevices(
) {
    // devicesManager = PreferencesManager(application)

    LazyVerticalGrid(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        columns = GridCells.Fixed(2),
        horizontalArrangement = Arrangement.spacedBy(15.dp),
        verticalArrangement = Arrangement.spacedBy(15.dp),
    ) {
        items(availableDevices) { device ->
            DeviceComponent(
                device,
                false
            )
        }
    }
}

fun onSharedPreferenceChanged(prefs: SharedPreferences?, key: String?) {
    availableDevices = devicesManager.get<List<MDnsDevice>>("devices").orEmpty()
}
