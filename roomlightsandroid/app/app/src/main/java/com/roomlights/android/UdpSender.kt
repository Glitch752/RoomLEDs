package com.roomlights.android

import android.content.Context
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.UnknownHostException

class UdpSender(
    context: Context,
    private val barsFlow: StateFlow<FloatArray>,
    private val udpEnabledFlow: StateFlow<Boolean>,
    private val udpAddressFlow: StateFlow<String>
) : Thread() {
    private val _status = MutableStateFlow("Idle")
    val status: StateFlow<String> = _status

    @Volatile
    private var running = true

    override fun run() {
        var socket: DatagramSocket? = null
        var lastAddress: String? = null
        var address: InetAddress?
        var port: Int
        var totalBytesSent: Long
        var totalPacketsSent: Long
        var lastRateTimestamp: Long

        while(running) {
            try {
                // Wait for UDP to be enabled
                runBlocking {
                    udpEnabledFlow.first { it }
                }

                // Get address and port
                val addrStr = udpAddressFlow.value
                val parts = addrStr.split(":")
                if (parts.size != 2) {
                    _status.value = "Error: Invalid UDP address format (expected host:port)"
                    Thread.sleep(500)
                    continue
                }
                val host = parts[0]
                port = try { parts[1].toInt() } catch (e: Exception) {
                    _status.value = "Error: Invalid port in UDP address"
                    Thread.sleep(500)
                    continue
                }
                try {
                    address = InetAddress.getByName(host)
                } catch (e: UnknownHostException) {
                    _status.value = "Error: Unknown host $host"
                    Thread.sleep(500)
                    continue
                }

                if(lastAddress != addrStr) {
                    socket?.close()
                    socket = DatagramSocket()
                    lastAddress = addrStr
                }

                // Main send loop
                totalBytesSent = 0L
                totalPacketsSent = 0L
                lastRateTimestamp = System.currentTimeMillis()
                while(udpEnabledFlow.value) {
                    val bars = barsFlow.value
                    // Convert float array to bytes (0-255)
                    val byteData = ByteArray(bars.size) { i ->
                        val v = (bars[i].coerceIn(0f, 1f) * 255f).toInt()
                        v.toByte()
                    }
                    val packet = DatagramPacket(byteData, byteData.size, address, port)
                    try {
                        socket?.send(packet)
                        totalBytesSent += byteData.size
                        totalPacketsSent++
                    } catch (e: Exception) {
                        _status.value = "Error: Failed to send UDP packet: ${e.message}"
                        break
                    }

                    // Update status every 100 packets
                    if(totalPacketsSent % 100L == 0L) {
                        val now = System.currentTimeMillis()
                        val elapsed = (now - lastRateTimestamp) / 1000.0
                        if (elapsed > 0) {
                            val bytesPerSec = totalBytesSent / elapsed
                            val packetsPerSec = 100.0 / elapsed
                            _status.value = String.format(
                                "Running: %.1f bytes/s (%.1f packets/s, %d packets total)",
                                bytesPerSec, packetsPerSec, totalPacketsSent
                            )
                        }
                        totalBytesSent = 0L
                        lastRateTimestamp = now
                    }

                    // Sleep to match framerate (80Hz = 12.5ms)
                    val sleepMs = 13L
                    Thread.sleep(sleepMs)
                }
                _status.value = "Idle"
            } catch (e: Exception) {
                _status.value = "Error: ${e.message}"
                Thread.sleep(1000)
            }
        }
        socket?.close()
    }

    fun stopSender() {
        running = false
        interrupt()
    }
}