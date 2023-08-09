package com.sergioribera.superclipboard.data

import androidx.room.*
import kotlinx.coroutines.flow.Flow
import uniffi.mdns.MDnsDevice

@Dao
interface DeviceDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertDevice(device: MDnsDevice)

    @Delete
    suspend fun deleteDevice(device: MDnsDevice)

    @Query("SELECT * FROM devices WHERE deviceId = :id")
    suspend fun getDeviceById(id: String): MDnsDevice?

    @Query("SELECT * FROM devices")
    fun getDevices(): Flow<List<MDnsDevice>>
}