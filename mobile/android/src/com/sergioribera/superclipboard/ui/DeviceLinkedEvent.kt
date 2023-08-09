package com.sergioribera.superclipboard.ui

import uniffi.mdns.MDnsDevice

sealed class DeviceLinkedEvent {
    data class OnDeleteDevice(val device: MDnsDevice): DeviceLinkedEvent()
    object OnUndoDeleteClick: DeviceLinkedEvent()
    data class OnAddDevice(val device: MDnsDevice): DeviceLinkedEvent()
}