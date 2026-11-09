package com.dexsentry.corebridge

import com.dexsentry.corebridge.generated.FfiException
import com.dexsentry.corebridge.generated.FfiScanReport
import com.dexsentry.corebridge.generated.FfiSeverity
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import com.dexsentry.corebridge.generated.analyzeApk as ffiAnalyzeApk
import com.dexsentry.corebridge.generated.version as ffiVersion

interface DexsentryEngine {
    fun version(): String

    suspend fun analyze(apkPath: String): Result<ScanReport>
}

data class ScanReport(
    val apkName: String?,
    val findings: List<Finding>,
    val riskScore: Int,
)

data class Finding(
    val id: String,
    val severity: Severity,
    val title: String,
    val evidence: List<String>,
)

enum class Severity {
    INFO,
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

sealed class EngineError(
    message: String,
) : Exception(message) {
    class NativeFailure(
        message: String,
    ) : EngineError(message)

    class Unexpected(
        cause: Exception,
    ) : EngineError(cause.message ?: "Unexpected engine error")
}

class RustDexsentryEngine : DexsentryEngine {
    override fun version(): String = ffiVersion()

    // FFI boundary: UniFFI reports a Rust panic as InternalException, which extends Exception,
    // and binding lift failures as RuntimeException. Neither may reach the UI as a raw exception.
    @Suppress("TooGenericExceptionCaught")
    override suspend fun analyze(apkPath: String): Result<ScanReport> =
        try {
            val report =
                withContext(Dispatchers.Default) {
                    ffiAnalyzeApk(apkPath).toDomain()
                }
            Result.success(report)
        } catch (cancellation: CancellationException) {
            throw cancellation
        } catch (error: FfiException) {
            Result.failure(EngineError.NativeFailure(error.message ?: "Rust analysis failed"))
        } catch (error: Exception) {
            Result.failure(EngineError.Unexpected(error))
        }
}

private fun FfiScanReport.toDomain(): ScanReport =
    ScanReport(
        apkName = apk.fileName,
        findings =
            findings.map { finding ->
                Finding(
                    id = finding.id,
                    severity = finding.severity.toDomain(),
                    title = finding.title,
                    evidence = finding.evidence,
                )
            },
        riskScore = riskScore.toInt(),
    )

private fun FfiSeverity.toDomain(): Severity =
    when (this) {
        FfiSeverity.INFO -> Severity.INFO
        FfiSeverity.LOW -> Severity.LOW
        FfiSeverity.MEDIUM -> Severity.MEDIUM
        FfiSeverity.HIGH -> Severity.HIGH
        FfiSeverity.CRITICAL -> Severity.CRITICAL
    }
