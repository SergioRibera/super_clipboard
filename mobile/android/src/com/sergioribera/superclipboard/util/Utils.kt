package com.sergioribera.superclipboard.util

import uniffi.mdns.ClipboardItem
import java.io.File
import java.io.FileOutputStream
import java.time.OffsetDateTime
import java.time.ZoneOffset
import java.time.format.DateTimeFormatter

fun getDateNowUtc(): String {
    val now = OffsetDateTime.now(ZoneOffset.UTC)
    return now.format(DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ssXXX"))
}