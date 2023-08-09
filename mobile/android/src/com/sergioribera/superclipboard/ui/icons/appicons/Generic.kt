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

public val AppIcons.Generic: ImageVector
    get() {
        if (_generic != null) {
            return _generic!!
        }
        _generic = Builder(name = "Generic", defaultWidth = 512.0.dp, defaultHeight = 512.0.dp,
                viewportWidth = 24.0f, viewportHeight = 24.0f).apply {
            path(fill = SolidColor(Color(0xFF768f9b)), stroke = null, strokeLineWidth = 0.0f,
                    strokeLineCap = Butt, strokeLineJoin = Miter, strokeLineMiter = 4.0f,
                    pathFillType = NonZero) {
                moveTo(20.0f, 8.0f)
                arcToRelative(2.0f, 2.0f, 0.0f, false, true, 1.995f, 1.85f)
                lineTo(22.0f, 10.0f)
                verticalLineToRelative(9.0f)
                arcToRelative(2.0f, 2.0f, 0.0f, false, true, -1.85f, 1.995f)
                lineTo(20.0f, 21.0f)
                horizontalLineToRelative(-4.0f)
                arcToRelative(2.0f, 2.0f, 0.0f, false, true, -1.995f, -1.85f)
                lineTo(14.0f, 19.0f)
                verticalLineToRelative(-9.0f)
                arcToRelative(2.0f, 2.0f, 0.0f, false, true, 1.85f, -1.995f)
                lineTo(16.0f, 8.0f)
                horizontalLineToRelative(4.0f)
                close()
                moveTo(17.0f, 4.0f)
                arcToRelative(2.0f, 2.0f, 0.0f, false, true, 2.0f, 2.0f)
                horizontalLineToRelative(-4.0f)
                arcToRelative(3.0f, 3.0f, 0.0f, false, false, -3.0f, 3.0f)
                verticalLineToRelative(11.0f)
                lineTo(4.0f, 20.0f)
                arcToRelative(2.0f, 2.0f, 0.0f, false, true, -2.0f, -2.0f)
                lineTo(2.0f, 6.0f)
                arcToRelative(2.0f, 2.0f, 0.0f, false, true, 2.0f, -2.0f)
                horizontalLineToRelative(13.0f)
                close()
            }
        }
        .build()
        return _generic!!
    }

private var _generic: ImageVector? = null
