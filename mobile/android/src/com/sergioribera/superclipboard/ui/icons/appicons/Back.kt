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

public val AppIcons.Back: ImageVector
    get() {
        if (_back != null) {
            return _back!!
        }
        _back = Builder(name = "Back", defaultWidth = 100.0.dp, defaultHeight = 100.0.dp,
                viewportWidth = 100.0f, viewportHeight = 100.0f).apply {
            path(fill = SolidColor(Color(0xFF778f9b)), stroke = SolidColor(Color(0x00000000)),
                    strokeLineWidth = 0.0f, strokeLineCap = Butt, strokeLineJoin = Miter,
                    strokeLineMiter = 4.0f, pathFillType = NonZero) {
                moveTo(68.913f, 8.586f)
                curveTo(71.149f, 6.49f, 73.646f, 6.008f, 75.29f, 8.042f)
                curveTo(77.028f, 10.192f, 76.079f, 12.511f, 74.005f, 14.555f)
                curveTo(63.108f, 25.299f, 52.504f, 36.348f, 41.412f, 46.884f)
                curveTo(37.749f, 50.364f, 38.274f, 52.072f, 41.6f, 55.263f)
                curveTo(52.402f, 65.625f, 62.859f, 76.347f, 73.438f, 86.941f)
                curveTo(75.371f, 88.877f, 78.143f, 90.997f, 75.441f, 93.824f)
                curveTo(72.695f, 96.698f, 70.128f, 94.669f, 67.861f, 92.392f)
                curveTo(55.775f, 80.248f, 43.636f, 68.156f, 31.544f, 56.018f)
                curveTo(27.799f, 52.259f, 27.818f, 49.722f, 31.61f, 45.92f)
                curveTo(43.945f, 33.555f, 56.31f, 21.219f, 68.913f, 8.586f)
                close()
            }
        }
        .build()
        return _back!!
    }

private var _back: ImageVector? = null
