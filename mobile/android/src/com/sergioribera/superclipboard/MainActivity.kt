package com.sergioribera.superclipboard

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.sergioribera.superclipboard.ui.theme.JetpackExampleTheme
import uniffi.mdns.ClipboardItem

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        var txt = uniffi.mdns.ClipboardItem.Text("", "Probando")
        setContent {
            JetpackExampleTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Greeting(txt)
                }
            }
        }
    }
}

@Composable
fun Greeting(data: uniffi.mdns.ClipboardItem, modifier: Modifier = Modifier) {
    var name = when(data) {
        is ClipboardItem.Image -> data.date
        is ClipboardItem.Text -> data.value
    }

    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    JetpackExampleTheme {
        Greeting(uniffi.mdns.ClipboardItem.Text("", "Preview"))
    }
}
