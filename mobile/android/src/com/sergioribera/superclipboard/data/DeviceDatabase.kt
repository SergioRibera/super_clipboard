package com.sergioribera.superclipboard.data

import android.content.Context
import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import uniffi.mdns.MDnsDevice

@Database(
    entities = [MDnsDevice::class],
    version = 1
)
abstract class DeviceDatabase : RoomDatabase() {
    abstract val dao: DeviceDao

    companion object {
        private var INSTANCE: DeviceDatabase? = null

        fun getInstance(context: Context): DeviceDatabase {
            if (INSTANCE == null) {
                INSTANCE = Room.databaseBuilder(context, DeviceDatabase::class.java, "device_db.db")
                    .build()
            }

            return INSTANCE!!
        }

        fun destroyInstance() {
            INSTANCE = null
        }
    }
}