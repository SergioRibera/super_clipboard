package com.sergioribera.superclipboard.data

import androidx.room.Database
import androidx.room.RoomDatabase
import uniffi.mdns.MDnsDevice

@Database(entities = [MDnsDevice::class],
    version = 1)
abstract class DeviceDatabase: RoomDatabase() {
    abstract val dao: DeviceDao
}