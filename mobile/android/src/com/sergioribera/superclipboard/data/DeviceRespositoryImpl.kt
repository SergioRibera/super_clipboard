package com.sergioribera.superclipboard.data

import kotlinx.coroutines.flow.Flow
import uniffi.mdns.MDnsDevice

class DeviceRespositoryImpl(private val dao: DeviceDao) : DeviceRepository {
    override suspend fun insertDevice(device: MDnsDevice) {
        dao.insertDevice(device)
    }

    override suspend fun deleteDevice(device: MDnsDevice) {
        dao.deleteDevice(device)
    }

    override suspend fun getDeviceById(id: String): MDnsDevice? {
        return dao.getDeviceById(id)
    }

    override fun getDevices(): Flow<List<MDnsDevice>> {
        return dao.getDevices()
    }
}