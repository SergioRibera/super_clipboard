package com.sergioribera.superclipboard.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Scaffold
import androidx.compose.material.rememberScaffoldState
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.sergioribera.superclipboard.ui.component.CustomBottomNavigation
import com.sergioribera.superclipboard.ui.layout.NotFound
import com.sergioribera.superclipboard.util.UiEvent

@Composable
fun Router(
    onNavigate: (UiEvent.Navigate) -> Unit,
) {
    val navController = rememberNavController()
    val scaffoldState = rememberScaffoldState()

    // LaunchedEffect(key1 = true) {
    //     viewModel.uiEvent.collect { event ->
    //         when (event) {
    //             is UiEvent.ShowSnackbar -> {
    //                 val result = scaffoldState.snackbarHostState.showSnackbar(
    //                     message = event.message,
    //                     actionLabel = event.action
    //                 )
    //                 if (result == SnackbarResult.ActionPerformed) {
    //                     viewModel.onEvent(DeviceLinkedEvent.OnUndoDeleteClick)
    //                 }
    //             }

    //             is UiEvent.Navigate -> onNavigate(event)
    //             else -> Unit
    //         }
    //     }
    // }


    Scaffold(
        scaffoldState = scaffoldState,
        bottomBar = { CustomBottomNavigation(navController = navController) },
        modifier = Modifier
            .fillMaxSize()
            .fillMaxSize()
            .background(MaterialTheme.colorScheme.background)
    ) {
        it.calculateTopPadding()

        NavHost(navController = navController, startDestination = "home") {
            composable("home") { AvailableDevices() }
            composable("linked") { LinkedDevices() }
            composable("settings") { NotFound() }
        }
    }
}