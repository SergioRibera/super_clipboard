package com.sergioribera.superclipboard.ui.layout

import android.content.Context
import android.content.SharedPreferences
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.sergioribera.superclipboard.ui.component.DeviceComponent
import uniffi.mdns.MDnsDevice

var devices: ArrayList<MDnsDevice> = ArrayList()
var sharedPref: SharedPreferences? = null

@Preview
@Composable
fun ShowDevices(only_linkeds: Boolean = false) {
    var devicesState: ArrayList<MDnsDevice> by rememberSaveable {
        mutableStateOf(devices)
    }

    LazyVerticalGrid(
        columns = GridCells.Fixed(2),
        horizontalArrangement = Arrangement.spacedBy(15.dp),
        verticalArrangement = Arrangement.spacedBy(15.dp)
    ) {
        items(devices) { device -> DeviceComponent(device, only_linkeds) }
    }
}