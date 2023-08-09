package com.sergioribera.superclipboard.ui.screens

import android.annotation.SuppressLint
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.material.Scaffold
import androidx.compose.material.SnackbarResult
import androidx.compose.material.rememberScaffoldState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import com.sergioribera.superclipboard.ui.DeviceLinkedEvent
import com.sergioribera.superclipboard.ui.DeviceLinkedViewModel
import com.sergioribera.superclipboard.ui.component.DeviceComponent
import com.sergioribera.superclipboard.util.UiEvent

@SuppressLint("UnusedMaterialScaffoldPaddingParameter")
@Composable
fun LinkedDevices(
    onNavigate: (UiEvent.Navigate) -> Unit,
    viewModel: DeviceLinkedViewModel = hiltViewModel()
) {
    val devices = viewModel.devices.collectAsState(initial = emptyList())
    val scaffoldState = rememberScaffoldState()

    LaunchedEffect(key1 = true) {
        viewModel.uiEvent.collect { event ->
            when (event) {
                is UiEvent.ShowSnackbar -> {
                    val result = scaffoldState.snackbarHostState.showSnackbar(
                        message = event.message,
                        actionLabel = event.action
                    )
                    if (result == SnackbarResult.ActionPerformed) {
                        viewModel.onEvent(DeviceLinkedEvent.OnUndoDeleteClick)
                    }
                }

                is UiEvent.Navigate -> onNavigate(event)
                else -> Unit
            }
        }
    }

    Scaffold(
        scaffoldState = scaffoldState,
    ) {
        LazyVerticalGrid(
            columns = GridCells.Fixed(2),
            horizontalArrangement = Arrangement.spacedBy(15.dp),
            verticalArrangement = Arrangement.spacedBy(15.dp),
        ) {
            items(devices.value) { device -> DeviceComponent(device, viewModel::onEvent, true) }
        }
    }

}