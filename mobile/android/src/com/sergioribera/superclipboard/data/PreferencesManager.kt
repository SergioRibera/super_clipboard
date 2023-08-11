package com.sergioribera.superclipboard.data

import android.app.Application
import android.content.Context
import android.content.SharedPreferences
import android.content.SharedPreferences.OnSharedPreferenceChangeListener
import com.google.gson.GsonBuilder
import com.google.gson.reflect.TypeToken
import java.lang.reflect.Type

class PreferencesManager(
    application: Application,
    fileName: String,
    listener: OnSharedPreferenceChangeListener?,
) {

    //Shared Preference field used to save and retrieve JSON string
    var preferences: SharedPreferences

    init {
        preferences = application.getSharedPreferences(
            fileName, Context.MODE_PRIVATE
        )
        if (listener != null)
            preferences.registerOnSharedPreferenceChangeListener(listener)
    }

    /**
     * Saves object into the Preferences.
     *
     * @param `object` Object of model class (of type [T]) to save
     * @param key Key with which Shared preferences to
     **/
    fun <T> put(`object`: T, key: String) {
        val jsonString = GsonBuilder().create().toJson(`object`)
        preferences.edit().putString(key, jsonString).apply()
    }

    /**
     * Used to retrieve object from the Preferences.
     *
     * @param key Shared Preference key with which object was saved.
     **/
    inline fun <reified T> get(key: String): T? {
        val value = preferences.getString(key, null)
        return GsonBuilder().create().fromJson(value, T::class.java)
    }

    /**
     * Used to retrieve object from the Preferences.
     *
     * @param key Shared Preference key with which object was saved.
     **/
    inline fun <reified T> getList(key: String): List<T> {
        val value = preferences.getString(key, null)
        val typeToken: Type = object : TypeToken<List<T>>() {}.type
        return GsonBuilder().create().fromJson(value, typeToken)
    }
}
