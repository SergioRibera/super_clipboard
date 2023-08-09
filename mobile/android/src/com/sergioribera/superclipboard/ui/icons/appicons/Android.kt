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

public val AppIcons.Android: ImageVector
    get() {
        if (_android != null) {
            return _android!!
        }
        _android = Builder(name = "Android", defaultWidth = 512.0.dp, defaultHeight = 512.0.dp,
                viewportWidth = 24.0f, viewportHeight = 24.0f).apply {
            path(fill = SolidColor(Color(0xFF768f9b)), stroke = null, strokeLineWidth = 0.0f,
                    strokeLineCap = Butt, strokeLineJoin = Miter, strokeLineMiter = 4.0f,
                    pathFillType = NonZero) {
                moveToRelative(14.975f, 3.019f)
                lineToRelative(0.96f, -1.732f)
                arcToRelative(0.193f, 0.193f, 0.0f, false, false, -0.338f, -0.187f)
                lineToRelative(-0.97f, 1.75f)
                arcToRelative(6.541f, 6.541f, 0.0f, false, false, -5.253f, 0.0f)
                lineToRelative(-0.97f, -1.75f)
                arcToRelative(0.193f, 0.193f, 0.0f, false, false, -0.34f, 0.187f)
                lineToRelative(0.96f, 1.732f)
                arcToRelative(5.546f, 5.546f, 0.0f, false, false, -3.092f, 4.876f)
                horizontalLineToRelative(12.137f)
                arcToRelative(5.546f, 5.546f, 0.0f, false, false, -3.094f, -4.876f)
                close()
                moveTo(9.2f, 5.674f)
                arcToRelative(0.507f, 0.507f, 0.0f, true, true, 0.507f, -0.506f)
                arcToRelative(0.507f, 0.507f, 0.0f, false, true, -0.507f, 0.506f)
                close()
                moveTo(14.802f, 5.674f)
                arcToRelative(0.507f, 0.507f, 0.0f, true, true, 0.507f, -0.506f)
                arcToRelative(0.507f, 0.507f, 0.0f, false, true, -0.507f, 0.506f)
                close()
                moveTo(5.93f, 17.171f)
                arcTo(1.467f, 1.467f, 0.0f, false, false, 7.4f, 18.64f)
                horizontalLineToRelative(0.973f)
                verticalLineToRelative(3.0f)
                arcToRelative(1.36f, 1.36f, 0.0f, true, false, 2.721f, 0.0f)
                verticalLineToRelative(-3.0f)
                horizontalLineToRelative(1.814f)
                verticalLineToRelative(3.0f)
                arcToRelative(1.36f, 1.36f, 0.0f, true, false, 2.72f, 0.0f)
                verticalLineToRelative(-3.0f)
                horizontalLineToRelative(0.974f)
                arcToRelative(1.467f, 1.467f, 0.0f, false, false, 1.468f, -1.468f)
                lineTo(18.07f, 8.375f)
                lineTo(5.93f, 8.375f)
                close()
                moveTo(4.063f, 8.141f)
                arcToRelative(1.362f, 1.362f, 0.0f, false, false, -1.36f, 1.361f)
                verticalLineToRelative(5.669f)
                arcToRelative(1.36f, 1.36f, 0.0f, true, false, 2.72f, 0.0f)
                lineTo(5.423f, 9.502f)
                arcToRelative(1.362f, 1.362f, 0.0f, false, false, -1.36f, -1.36f)
                close()
                moveTo(19.935f, 8.141f)
                arcToRelative(1.362f, 1.362f, 0.0f, false, false, -1.36f, 1.361f)
                verticalLineToRelative(5.669f)
                arcToRelative(1.36f, 1.36f, 0.0f, true, false, 2.72f, 0.0f)
                lineTo(21.295f, 9.502f)
                arcToRelative(1.362f, 1.362f, 0.0f, false, false, -1.36f, -1.36f)
                close()
            }
        }
        .build()
        return _android!!
    }

private var _android: ImageVector? = null
