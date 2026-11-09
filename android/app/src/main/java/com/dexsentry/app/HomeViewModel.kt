package com.dexsentry.app

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import com.dexsentry.corebridge.DexsentryEngine
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class HomeUiState(
    val coreVersion: String? = null,
)

class HomeViewModel(
    private val engine: DexsentryEngine,
) : ViewModel() {
    private val mutableUiState = MutableStateFlow(HomeUiState())
    val uiState = mutableUiState.asStateFlow()

    init {
        viewModelScope.launch(Dispatchers.Default) {
            mutableUiState.value = HomeUiState(coreVersion = engine.version())
        }
    }

    class Factory(
        private val engine: DexsentryEngine,
    ) : ViewModelProvider.Factory {
        override fun <T : ViewModel> create(modelClass: Class<T>): T {
            require(modelClass.isAssignableFrom(HomeViewModel::class.java)) {
                "Unsupported ViewModel type: ${modelClass.name}"
            }
            return modelClass.cast(HomeViewModel(engine))
        }
    }
}
