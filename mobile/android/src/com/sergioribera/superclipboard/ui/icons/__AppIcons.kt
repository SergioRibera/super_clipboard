package icons

import androidx.compose.ui.graphics.vector.ImageVector
import icons.appicons.Android
import icons.appicons.Apple
import icons.appicons.Back
import icons.appicons.Generic
import icons.appicons.Hint
import icons.appicons.Linux
import icons.appicons.Password
import icons.appicons.Pin
import icons.appicons.Settings
import icons.appicons.Trash
import icons.appicons.Unpin
import icons.appicons.Windows
import icons.appicons.`Light-mode`
import icons.appicons.`Night-mode`
import kotlin.collections.List as ____KtList

public object AppIcons

private var __AllIcons: ____KtList<ImageVector>? = null

public val AppIcons.AllIcons: ____KtList<ImageVector>
  get() {
    if (__AllIcons != null) {
      return __AllIcons!!
    }
    __AllIcons= listOf(`Light-mode`, `Night-mode`, Back, Password, Pin, Unpin, Linux, Apple,
        Windows, Android, Generic, Trash, Settings, Hint)
    return __AllIcons!!
  }
