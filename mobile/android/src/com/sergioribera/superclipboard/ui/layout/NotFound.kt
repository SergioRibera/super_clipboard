package com.sergioribera.superclipboard.ui.layout

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable

@Composable
fun NotFound() {
    LazyColumn {
        item { Text(text = "Content Not Found") }
    }
}