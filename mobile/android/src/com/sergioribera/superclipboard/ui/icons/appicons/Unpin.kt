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

public val AppIcons.Unpin: ImageVector
    get() {
        if (_unpin != null) {
            return _unpin!!
        }
        _unpin = Builder(name = "Unpin", defaultWidth = 100.0.dp, defaultHeight = 100.0.dp,
                viewportWidth = 100.0f, viewportHeight = 100.0f).apply {
            path(fill = SolidColor(Color(0xFF768f9b)), stroke = null, fillAlpha = 0.954f,
                    strokeAlpha = 0.954f, strokeLineWidth = 0.0f, strokeLineCap = Butt,
                    strokeLineJoin = Miter, strokeLineMiter = 4.0f, pathFillType = NonZero) {
                moveTo(35.5f, 7.5f)
                curveTo(37.467f, 7.261f, 39.3f, 7.594f, 41.0f, 8.5f)
                curveTo(41.54f, 11.419f, 41.207f, 14.086f, 40.0f, 16.5f)
                curveTo(45.167f, 21.667f, 50.333f, 26.833f, 55.5f, 32.0f)
                curveTo(60.5f, 32.333f, 65.5f, 32.667f, 70.5f, 33.0f)
                curveTo(74.805f, 33.903f, 78.639f, 35.736f, 82.0f, 38.5f)
                curveTo(84.0f, 41.833f, 84.0f, 45.167f, 82.0f, 48.5f)
                curveTo(77.318f, 53.015f, 72.818f, 57.681f, 68.5f, 62.5f)
                curveTo(75.152f, 69.652f, 81.985f, 76.652f, 89.0f, 83.5f)
                curveTo(90.5f, 88.667f, 88.667f, 90.5f, 83.5f, 89.0f)
                curveTo(76.652f, 81.985f, 69.652f, 75.152f, 62.5f, 68.5f)
                curveTo(57.681f, 72.818f, 53.015f, 77.318f, 48.5f, 82.0f)
                curveTo(44.634f, 84.12f, 40.967f, 83.786f, 37.5f, 81.0f)
                curveTo(33.165f, 73.098f, 31.332f, 64.598f, 32.0f, 55.5f)
                curveTo(26.333f, 49.833f, 20.667f, 44.167f, 15.0f, 38.5f)
                curveTo(13.419f, 41.553f, 11.086f, 42.22f, 8.0f, 40.5f)
                curveTo(7.333f, 38.833f, 7.333f, 37.167f, 8.0f, 35.5f)
                curveTo(15.36f, 24.307f, 24.527f, 14.974f, 35.5f, 7.5f)
                close()
                moveTo(31.5f, 20.5f)
                curveTo(38.609f, 26.436f, 45.609f, 32.603f, 52.5f, 39.0f)
                curveTo(54.0f, 39.5f, 55.5f, 40.0f, 57.0f, 40.5f)
                curveTo(63.651f, 38.701f, 69.817f, 39.701f, 75.5f, 43.5f)
                curveTo(65.167f, 54.5f, 54.5f, 65.167f, 43.5f, 75.5f)
                curveTo(41.52f, 73.205f, 40.353f, 70.538f, 40.0f, 67.5f)
                curveTo(39.667f, 62.5f, 39.333f, 57.5f, 39.0f, 52.5f)
                curveTo(32.865f, 45.364f, 26.365f, 38.531f, 19.5f, 32.0f)
                curveTo(23.395f, 27.935f, 27.395f, 24.102f, 31.5f, 20.5f)
                close()
            }
        }
        .build()
        return _unpin!!
    }

private var _unpin: ImageVector? = null
