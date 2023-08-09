package com.sergioribera.superclipboard.ui.component

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.sergioribera.superclipboard.ui.theme.AppTheme
import com.sergioribera.superclipboard.ui.theme.Gunmetal
import com.sergioribera.superclipboard.ui.theme.PowderBlue
import icons.AppIcons
import icons.appicons.Android
import icons.appicons.Apple
import icons.appicons.Generic
import icons.appicons.Linux
import icons.appicons.Windows
import uniffi.mdns.MDnsDevice

@OptIn(ExperimentalMaterial3Api::class, ExperimentalFoundationApi::class)
@Composable
fun DeviceComponent(device: MDnsDevice, isLinked: Boolean) {
    val icon = when (device.os) {
        "android" -> AppIcons.Android
        "macos" -> AppIcons.Apple
        "windows" -> AppIcons.Windows
        "linux" -> AppIcons.Linux
        else -> AppIcons.Generic
    }
    var requestSended by rememberSaveable {
        mutableStateOf(false)
    }
    val tint = if (isLinked) {
        PowderBlue
    } else if (requestSended) {
        MaterialTheme.colorScheme.onSecondary
    } else {
        Gunmetal
    }

    Card(
        onClick = {
        },
        modifier = Modifier
            .fillMaxWidth()
            .fillMaxHeight()
            .padding(15.dp)
            .combinedClickable(
                onClick = {
                    if (!isLinked && !requestSended) {
                        // TODO: send other device request to link
                        requestSended = true
                    }
                },
                onLongClick = {
                    if (isLinked) {
                        // TODO: Send Remove Linked Request to other device
                    }
                }
            ),
        enabled = !requestSended,
        ) {
        LazyColumn(horizontalAlignment = Alignment.CenterHorizontally) {
            item {
                Icon(
                    icon,
                    modifier = Modifier
                        .fillMaxSize((6.0 / 8.0).toFloat())
                        .padding(20.dp),
                    contentDescription = null,
                    tint = tint
                )
            }
            item {
                Text(
                    device.name,
                    Modifier
                        .fillMaxWidth()
                        .padding(10.dp, 0.dp),
                    textAlign = TextAlign.Center,
                    overflow = TextOverflow.Ellipsis,
                )
            }
        }
    }
}

@Preview
@Composable
fun PreviewDeviceComponentWindowsIcon() {
    AppTheme {
        DeviceComponent( MDnsDevice("123", "La Poderosa", "android"), false)
    }
}