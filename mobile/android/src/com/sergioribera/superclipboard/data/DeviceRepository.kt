package com.sergioribera.superclipboard.data

import kotlinx.coroutines.flow.Flow
import uniffi.mdns.MDnsDevice

interface DeviceRepository {

    suspend fun insertDevice(device: MDnsDevice)

    suspend fun deleteDevice(device: MDnsDevice)

    suspend fun getDeviceById(id: String): MDnsDevice?

    fun getDevices(): Flow<List<MDnsDevice>>
}