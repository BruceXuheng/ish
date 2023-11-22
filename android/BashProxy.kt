
class BashProxy(private val port: Int) {

    companion object {

        private val TAG = BashProxy::class.java.simpleName

        fun runAdbCommand(port: Int, adbCmd: String, serialNo: String): String {
            val bashProxy = BashProxy(port)
            return bashProxy.runAdbCommand(adbCmd, serialNo)
        }

    }

    @Throws(IOException::class)
    fun execute(cmd: String): String {
        try {
            Socket().use { socket ->
                socket.connect(InetSocketAddress("127.0.0.1", port))
                //10秒超时
                socket.soTimeout = 10 * 1000
                socket.getOutputStream().use { os ->
                    socket.getInputStream().use { `is` ->

                        val request = ClientRequest.newBuilder()
                            .setSerialNo("11111111")
                            .setShellRequest(ShellRequest.newBuilder()
                                .setCmdStr("adb devices")
                                .build())
                            .build()
                        request.writeDelimitedTo(os)
                        os.write("\r\n\r\n".toByteArray(charset("utf-8")))
                        os.flush()

                        val response = ServerResponse.parseDelimitedFrom(`is`)

                        return response.toString()
                    }
                }
            }
        } catch (e: IOException) {
            throw e
        }
    }

    fun runAdbCommand(adbCmd: String, serialNo: String): String {
        return try {
            var sn = getSN()
            if (TextUtils.isEmpty(sn)) {
                sn = serialNo
            }
            if (TextUtils.isEmpty(sn)) {
                throw Exception("sn获取失败")
            }
            val cmd = "adb -s $sn $adbCmd\nexit\n"
            PLog.i(TAG, cmd)
            val output = execute(cmd)
            PLog.i(TAG, output)
            output
        } catch (e: Exception) {
            PLog.e(TAG, "bash转发失败 ${e.message}")
            ""
        }
    }

}