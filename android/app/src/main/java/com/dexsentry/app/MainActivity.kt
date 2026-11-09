package com.dexsentry.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.dexsentry.corebridge.RustDexsentryEngine
import com.dexsentry.features.report.ReportScreen
import com.dexsentry.features.scan.ScanScreen

class MainActivity : ComponentActivity() {
    private val engine = RustDexsentryEngine()
    private val homeViewModel: HomeViewModel by viewModels { HomeViewModel.Factory(engine) }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            val state by homeViewModel.uiState.collectAsStateWithLifecycle()
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    DexsentryApp(coreVersion = state.coreVersion)
                }
            }
        }
    }
}

private enum class AppScreen {
    HOME,
    SCAN,
    REPORT,
}

@Composable
@Suppress("FunctionNaming", "ktlint:standard:function-naming")
private fun DexsentryApp(coreVersion: String?) {
    var currentScreen by remember { mutableStateOf(AppScreen.HOME) }
    when (currentScreen) {
        AppScreen.HOME -> {
            HomeScreen(
                coreVersion = coreVersion,
                onScan = { currentScreen = AppScreen.SCAN },
                onReport = { currentScreen = AppScreen.REPORT },
            )
        }

        AppScreen.SCAN -> {
            ScanScreen(onBack = { currentScreen = AppScreen.HOME })
        }

        AppScreen.REPORT -> {
            ReportScreen(onBack = { currentScreen = AppScreen.HOME })
        }
    }
}

@Composable
@Suppress("FunctionNaming", "ktlint:standard:function-naming")
private fun HomeScreen(
    coreVersion: String?,
    onScan: () -> Unit,
    onReport: () -> Unit,
) {
    Column(
        modifier = Modifier.fillMaxSize().padding(24.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center,
    ) {
        Text("Dexsentry", style = MaterialTheme.typography.headlineLarge)
        Text(coreVersion?.let { "Dexsentry core v$it" } ?: "Connecting to Dexsentry core…")
        Button(onClick = onScan) {
            Text("Scan APK")
        }
        Button(onClick = onReport) {
            Text("View report")
        }
    }
}
