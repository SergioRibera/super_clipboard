package icons.appicons

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.PathFillType.Companion.NonZero
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.graphics.StrokeCap.Companion.Butt
import androidx.compose.ui.graphics.StrokeJoin.Companion.Miter
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.graphics.vector.ImageVector.Builder
import androidx.compose.ui.graphics.vector.path
import androidx.compose.ui.unit.dp
import icons.AppIcons

public val AppIcons.Hint: ImageVector
    get() {
        if (_hint != null) {
            return _hint!!
        }
        _hint = Builder(name = "Hint", defaultWidth = 48.0.dp, defaultHeight = 48.0.dp,
                viewportWidth = 256.0f, viewportHeight = 256.0f).apply {
            path(fill = SolidColor(Color(0xFF778f9b)), stroke = SolidColor(Color(0x00000000)),
                    strokeLineWidth = 1.0f, strokeLineCap = Butt, strokeLineJoin = Miter,
                    strokeLineMiter = 4.0f, pathFillType = NonZero) {
                moveTo(128.0f, 21.417f)
                curveToRelative(-41.121f, 0.012f, -74.667f, 33.447f, -74.667f, 74.583f)
                curveToRelative(0.0f, 21.33f, 9.021f, 40.637f, 23.406f, 54.229f)
                curveToRelative(1.692f, 1.598f, 2.935f, 3.848f, 3.51f, 6.427f)
                lineToRelative(13.156f, 59.219f)
                curveToRelative(2.43f, 10.937f, 12.223f, 18.792f, 23.427f, 18.792f)
                lineToRelative(22.333f, 0.0f)
                curveToRelative(11.205f, 0.0f, 20.996f, -7.855f, 23.427f, -18.792f)
                lineToRelative(13.167f, -59.219f)
                lineToRelative(0.0f, -0.01f)
                curveToRelative(0.571f, -2.571f, 1.806f, -4.809f, 3.5f, -6.406f)
                lineToRelative(0.0f, -0.01f)
                lineToRelative(0.01f, 0.0f)
                curveToRelative(14.379f, -13.591f, 23.396f, -32.899f, 23.396f, -54.229f)
                curveToRelative(0.0f, -41.136f, -33.545f, -74.571f, -74.667f, -74.583f)
                close()
                moveTo(128.0f, 37.417f)
                curveToRelative(32.5f, 0.009f, 58.667f, 26.098f, 58.667f, 58.583f)
                curveToRelative(0.0f, 16.835f, -7.068f, 31.897f, -18.396f, 42.604f)
                curveToRelative(-4.252f, 4.012f, -6.936f, 9.18f, -8.135f, 14.583f)
                lineToRelative(-6.25f, 28.146f)
                lineToRelative(-51.771f, 0.0f)
                lineToRelative(-6.25f, -28.156f)
                curveToRelative(-1.206f, -5.4f, -3.886f, -10.56f, -8.135f, -14.573f)
                curveToRelative(-11.332f, -10.707f, -18.396f, -25.769f, -18.396f, -42.604f)
                curveToRelative(0.0f, -32.485f, 26.167f, -58.574f, 58.667f, -58.583f)
                close()
                moveTo(105.677f, 197.333f)
                lineToRelative(44.646f, 0.0f)
                lineToRelative(-3.344f, 15.073f)
                curveToRelative(-0.822f, 3.698f, -4.02f, 6.26f, -7.812f, 6.26f)
                lineToRelative(-22.333f, 0.0f)
                curveToRelative(-3.804f, 0.0f, -6.989f, -2.552f, -7.812f, -6.26f)
                close()
            }
        }
        .build()
        return _hint!!
    }

private var _hint: ImageVector? = null
