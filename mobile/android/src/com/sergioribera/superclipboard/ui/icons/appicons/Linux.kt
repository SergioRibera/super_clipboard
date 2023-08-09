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

public val AppIcons.Linux: ImageVector
    get() {
        if (_linux != null) {
            return _linux!!
        }
        _linux = Builder(name = "Linux", defaultWidth = 512.0.dp, defaultHeight = 512.0.dp,
                viewportWidth = 24.0f, viewportHeight = 24.0f).apply {
            path(fill = SolidColor(Color(0xFF768f9b)), stroke = null, strokeLineWidth = 0.0f,
                    strokeLineCap = Butt, strokeLineJoin = Miter, strokeLineMiter = 4.0f,
                    pathFillType = NonZero) {
                moveTo(19.0f, 16.0f)
                curveToRelative(0.0f, 1.72f, -0.63f, 3.3f, -1.66f, 4.5f)
                curveToRelative(0.41f, 0.39f, 0.66f, 0.91f, 0.66f, 1.5f)
                horizontalLineTo(6.0f)
                curveToRelative(0.0f, -0.59f, 0.25f, -1.11f, 0.66f, -1.5f)
                arcTo(6.902f, 6.902f, 0.0f, false, true, 5.0f, 16.0f)
                horizontalLineTo(3.0f)
                curveToRelative(0.0f, -1.25f, 0.57f, -2.36f, 1.46f, -3.09f)
                lineToRelative(0.01f, -0.02f)
                arcTo(6.004f, 6.004f, 0.0f, false, false, 7.0f, 8.0f)
                verticalLineTo(7.0f)
                arcToRelative(5.0f, 5.0f, 0.0f, false, true, 5.0f, -5.0f)
                arcToRelative(5.0f, 5.0f, 0.0f, false, true, 5.0f, 5.0f)
                verticalLineToRelative(1.0f)
                curveToRelative(0.0f, 2.0f, 1.0f, 3.81f, 2.53f, 4.89f)
                lineToRelative(0.01f, 0.02f)
                curveToRelative(0.89f, 0.73f, 1.46f, 1.84f, 1.46f, 3.09f)
                horizontalLineToRelative(-2.0f)
                moveToRelative(-3.0f, 0.0f)
                arcToRelative(4.0f, 4.0f, 0.0f, false, false, -4.0f, -4.0f)
                arcToRelative(4.0f, 4.0f, 0.0f, false, false, -4.0f, 4.0f)
                arcToRelative(4.0f, 4.0f, 0.0f, false, false, 4.0f, 4.0f)
                arcToRelative(4.0f, 4.0f, 0.0f, false, false, 4.0f, -4.0f)
                moveToRelative(-6.0f, -7.0f)
                lineToRelative(2.0f, 1.5f)
                lineTo(14.0f, 9.0f)
                lineToRelative(-2.0f, -1.5f)
                lineTo(10.0f, 9.0f)
                moveToRelative(0.0f, -4.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, -1.0f, 1.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, 1.0f, 1.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, 1.0f, -1.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, -1.0f, -1.0f)
                moveToRelative(4.0f, 0.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, -1.0f, 1.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, 1.0f, 1.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, 1.0f, -1.0f)
                arcToRelative(1.0f, 1.0f, 0.0f, false, false, -1.0f, -1.0f)
                close()
            }
        }
        .build()
        return _linux!!
    }

private var _linux: ImageVector? = null
