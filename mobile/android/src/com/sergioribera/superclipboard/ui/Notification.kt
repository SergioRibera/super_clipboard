package com.sergioribera.superclipboard.ui

import android.content.Context
import androidx.core.app.NotificationCompat
import com.sergioribera.superclipboard.R
import dagger.Provides
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Singleton

@Singleton
@Provides
fun provideNotificationBuilder(
    @ApplicationContext context: Context,
    channelId: String,
    title: String,
    body: String,
): NotificationCompat.Builder {
    return NotificationCompat.Builder(context, channelId)
        .setContentTitle(title)
        .setContentText(body)
        .setSmallIcon(R.mipmap.ic_launcher)
        .setPriority(NotificationCompat.PRIORITY_DEFAULT)
        .addAction(0, "Cancel", null)
        .addAction(0, "Accept", null)
}