package com.sergioribera.superclipboard.services

import android.app.Service
import android.content.ClipData
import android.content.ClipboardManager
import android.content.Intent
import android.content.SharedPreferences
import android.net.wifi.WifiManager
import android.os.IBinder
import android.os.PowerManager
import android.util.Log
import androidx.core.net.toFile
import com.sergioribera.superclipboard.data.PreferencesManager
import com.sergioribera.superclipboard.ui.provideNotificationBuilder
import com.sergioribera.superclipboard.util.getDateNowUtc
import uniffi.mdns.ClipboardItem
import uniffi.mdns.MDnsDevice
import uniffi.mdns.MDnsMessage
import uniffi.mdns.decodeMessage
import uniffi.mdns.encodeMessage
import java.net.DatagramPacket
import java.net.Inet4Address
import java.net.InetAddress
import java.net.MulticastSocket

class UDPService : Service() {

    private val multicastAddress: InetAddress = Inet4Address.getByName("239.255.42.98")
    private val port: Int = 50692
    private val bufferSize: Int = 4048
    private lateinit var socket: MulticastSocket

    private lateinit var multicastLock: WifiManager.MulticastLock
    // private lateinit var wakeLock: PowerManager.WakeLock

    private lateinit var myDevice: MDnsDevice
    private lateinit var clipboardManager: ClipboardManager
    private lateinit var devicesManager: PreferencesManager

    private var availableDevices: MutableList<MDnsDevice> = listOf<MDnsDevice>().toMutableList()

    override fun onCreate() {
        super.onCreate()
        devicesManager = PreferencesManager(application, "devices", listener = null)
        val powerManager = getSystemService(POWER_SERVICE) as PowerManager
        // wakeLock =
        //    powerManager.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "MulticastService:WakeLock")
        clipboardManager = (this.getSystemService(CLIPBOARD_SERVICE) as ClipboardManager)
        clipboardManager.addPrimaryClipChangedListener {
            onChangeClipboard()
        }
        myDevice = PreferencesManager(application, "devices", listener = null).get("me")!!

        var wifi = applicationContext.getSystemService(WIFI_SERVICE) as WifiManager
        multicastLock = wifi.createMulticastLock("superclipboard")
        multicastLock.setReferenceCounted(true)
    }

    override fun onBind(intent: Intent): IBinder? {
        return null
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        multicastLock.acquire()
        // wakeLock.acquire()

        Thread {
            socket = MulticastSocket(port)
            socket.joinGroup(multicastAddress)
            devicesManager.put(emptyList<MDnsDevice>(), "devices")

            val msg = MDnsMessage.Connected(MDnsDevice("", "", "android"))
            val bytes = encodeMessage(msg)
            val hiNetwork = DatagramPacket(
                bytes, bytes.size,
                multicastAddress, port
            );
            socket.send(hiNetwork);

            // get their responses!
            while (true) {
                val receiveBuffer = ByteArray(bufferSize)
                val receivePacket = DatagramPacket(receiveBuffer, receiveBuffer.size)
                socket.receive(receivePacket)
                val msg = decodeMessage(receiveBuffer)

                Log.i("udp::service", "$msg")
                when (msg) {
                    is MDnsMessage.Connected -> {
                        addToAvailableDevices(msg.device)
                    }

                    is MDnsMessage.Welcome -> {
                        if (msg.to.deviceId == myDevice.deviceId) {
                            addToAvailableDevices(msg.from)
                        }
                    }

                    is MDnsMessage.LinkRequest -> {
                        showNotification(msg)
                    }

                    is MDnsMessage.LinkAccepted -> {
                        acceptLinkRequest(msg)
                    }

                    is MDnsMessage.Clipboard -> {
                        addToClipboard(msg)
                    }

                    else -> {}
                }
            }
        }.start()

        return START_STICKY
    }

    fun sendMessage(msg: MDnsMessage) {
        val bytes = encodeMessage(msg)
        val hiNetwork = DatagramPacket(
            bytes, bytes.size,
            multicastAddress, port
        );
        socket.send(hiNetwork);
    }

    fun onChangeClipboard() {
        if (clipboardManager.primaryClip == null) return
        val item = clipboardManager.primaryClip!!.getItemAt(0)
        if (!item.text.isNullOrEmpty()) {
            val msg = MDnsMessage.Clipboard(
                myDevice,
                ClipboardItem.Text(getDateNowUtc(), item.text.toString())
            )
            sendMessage(msg)
        }
        if (item.uri != null) {
            val file = item.uri.toFile()
            if (file.extension.matches(Regex("png|jpg"))) {
                val clipboardItem = ClipboardItem.Image(
                    getDateNowUtc(),
                    0 as ULong,
                    0 as ULong,
                    file.readBytes().map { b -> b as UByte }.toList()
                )
                val msg = MDnsMessage.Clipboard(myDevice, clipboardItem)
                sendMessage(msg)
            }
        }
    }

    // TODO: check if device send message is linked

    private fun addToAvailableDevices(device: MDnsDevice) {
        if (device.deviceId.isEmpty() || device.deviceId == myDevice.deviceId) return
        // TODO: check if device send message is linked
        if (!availableDevices.contains(device)) {
            availableDevices.add(device)
            devicesManager.put(availableDevices, "devices")
        }
    }

    private fun showNotification(msg: MDnsMessage.LinkRequest) {
        if (msg.to.deviceId != myDevice.deviceId) return
        provideNotificationBuilder(
            this,
            msg.from.deviceId,
            "Linked Request (${msg.from.name})",
            "\"${msg.from.name}\" request linked with you"
        )
    }

    private fun acceptLinkRequest(msg: MDnsMessage.LinkAccepted) {
        // TODO: add device to linkedDevices Database
    }

    private fun addToClipboard(msg: MDnsMessage.Clipboard) {
        when (msg.item) {
            is ClipboardItem.Text -> {
                clipboardManager.setPrimaryClip(
                    ClipData.newPlainText(
                        msg.item.value,
                        msg.item.value
                    )
                )
            }

            is ClipboardItem.Image -> {
                // TODO: implement copy image
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        socket.leaveGroup(multicastAddress)
        socket.close()
        // wakeLock.release()
        multicastLock.release()
    }
}