package com.sergioribera.superclipboard.di

import android.app.Application
import androidx.room.Room
import com.sergioribera.superclipboard.data.DeviceDatabase
import com.sergioribera.superclipboard.data.DeviceRepository
import com.sergioribera.superclipboard.data.DeviceRespositoryImpl
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object AppModule {

    @Provides
    @Singleton
    fun provideDeviceDatabase(app: Application): DeviceDatabase {
        return Room.databaseBuilder(
            app,
            DeviceDatabase::class.java,
            "device_db"
        ).build()
    }

    @Provides
    @Singleton
    fun provideDeviceRepository(db: DeviceDatabase): DeviceRepository {
        return DeviceRespositoryImpl(db.dao)
    }
}