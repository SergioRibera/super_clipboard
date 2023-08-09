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

public val AppIcons.Windows: ImageVector
    get() {
        if (_windows != null) {
            return _windows!!
        }
        _windows = Builder(name = "Windows", defaultWidth = 512.0.dp, defaultHeight = 512.0.dp,
                viewportWidth = 15.0f, viewportHeight = 15.0f).apply {
            path(fill = SolidColor(Color(0xFF768f9b)), stroke = null, strokeLineWidth = 0.0f,
                    strokeLineCap = Butt, strokeLineJoin = Miter, strokeLineMiter = 4.0f,
                    pathFillType = NonZero) {
                moveTo(14.814f, 0.111f)
                arcTo(0.5f, 0.5f, 0.0f, false, true, 15.0f, 0.5f)
                lineTo(15.0f, 7.0f)
                lineTo(7.0f, 7.0f)
                lineTo(7.0f, 1.596f)
                lineTo(14.395f, 0.01f)
                arcToRelative(0.5f, 0.5f, 0.0f, false, true, 0.42f, 0.1f)
                close()
                moveTo(6.0f, 1.81f)
                lineTo(0.395f, 3.011f)
                arcTo(0.5f, 0.5f, 0.0f, false, false, 0.0f, 3.5f)
                lineTo(0.0f, 7.0f)
                horizontalLineToRelative(6.0f)
                lineTo(6.0f, 1.81f)
                close()
                moveTo(0.0f, 8.0f)
                verticalLineToRelative(4.5f)
                arcToRelative(0.5f, 0.5f, 0.0f, false, false, 0.43f, 0.495f)
                lineToRelative(5.57f, 0.796f)
                lineTo(6.0f, 8.0f)
                lineTo(0.0f, 8.0f)
                close()
                moveTo(7.0f, 13.934f)
                lineToRelative(7.43f, 1.061f)
                arcTo(0.5f, 0.5f, 0.0f, false, false, 15.0f, 14.5f)
                lineTo(15.0f, 8.0f)
                lineTo(7.0f, 8.0f)
                verticalLineToRelative(5.934f)
                close()
            }
        }
        .build()
        return _windows!!
    }

private var _windows: ImageVector? = null
