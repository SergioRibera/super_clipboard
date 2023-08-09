package com.sergioribera.superclipboard.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.sergioribera.superclipboard.data.DeviceRepository
import com.sergioribera.superclipboard.util.UiEvent
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.launch
import uniffi.mdns.MDnsDevice
import javax.inject.Inject

@HiltViewModel
class DeviceLinkedViewModel @Inject constructor(
    private val repository: DeviceRepository
) : ViewModel() {

    val devices = repository.getDevices()

    private val _uiEvent = Channel<UiEvent>()
    val uiEvent = _uiEvent.receiveAsFlow()

    private var deletedTodo: MDnsDevice? = null

    fun onEvent(event: DeviceLinkedEvent) {
        when (event) {
            is DeviceLinkedEvent.OnUndoDeleteClick -> {
                deletedTodo?.let { device ->
                    viewModelScope.launch {
                        repository.insertDevice(device)
                    }
                }
            }

            is DeviceLinkedEvent.OnDeleteDevice -> {
                viewModelScope.launch {
                    deletedTodo = event.device
                    repository.deleteDevice(event.device)
                    sendUiEvent(
                        UiEvent.ShowSnackbar(
                            message = "Device unlinked",
                            action = "Undo"
                        )
                    )
                }
            }

            is DeviceLinkedEvent.OnAddDevice -> {
                viewModelScope.launch {
                    repository.insertDevice(
                        event.device
                    )
                }
            }
        }
    }

    private fun sendUiEvent(event: UiEvent) {
        viewModelScope.launch {
            _uiEvent.send(event)
        }
    }
}