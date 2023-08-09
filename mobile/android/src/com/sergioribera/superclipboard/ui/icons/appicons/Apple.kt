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

public val AppIcons.Apple: ImageVector
    get() {
        if (_apple != null) {
            return _apple!!
        }
        _apple = Builder(name = "Apple", defaultWidth = 512.0.dp, defaultHeight = 512.0.dp,
                viewportWidth = 26.0f, viewportHeight = 26.0f).apply {
            path(fill = SolidColor(Color(0xFF768f9b)), stroke = null, strokeLineWidth = 0.0f,
                    strokeLineCap = Butt, strokeLineJoin = Miter, strokeLineMiter = 4.0f,
                    pathFillType = NonZero) {
                moveTo(23.934f, 18.947f)
                curveToRelative(-0.598f, 1.324f, -0.884f, 1.916f, -1.652f, 3.086f)
                curveToRelative(-1.073f, 1.634f, -2.588f, 3.673f, -4.461f, 3.687f)
                curveToRelative(-1.666f, 0.014f, -2.096f, -1.087f, -4.357f, -1.069f)
                curveToRelative(-2.261f, 0.011f, -2.732f, 1.089f, -4.4f, 1.072f)
                curveToRelative(-1.873f, -0.017f, -3.307f, -1.854f, -4.381f, -3.485f)
                curveToRelative(-3.003f, -4.575f, -3.32f, -9.937f, -1.464f, -12.79f)
                curveTo(4.532f, 7.425f, 6.61f, 6.237f, 8.561f, 6.237f)
                curveToRelative(1.987f, 0.0f, 3.236f, 1.092f, 4.879f, 1.092f)
                curveToRelative(1.594f, 0.0f, 2.565f, -1.095f, 4.863f, -1.095f)
                curveToRelative(1.738f, 0.0f, 3.576f, 0.947f, 4.889f, 2.581f)
                curveToRelative(-4.296f, 2.354f, -3.598f, 8.49f, 0.742f, 10.132f)
                close()
                moveTo(16.559f, 4.408f)
                curveToRelative(0.836f, -1.073f, 1.47f, -2.587f, 1.24f, -4.131f)
                curveToRelative(-1.364f, 0.093f, -2.959f, 0.964f, -3.891f, 2.092f)
                curveToRelative(-0.844f, 1.027f, -1.544f, 2.553f, -1.271f, 4.029f)
                curveToRelative(1.488f, 0.048f, 3.028f, -0.839f, 3.922f, -1.99f)
                close()
            }
        }
        .build()
        return _apple!!
    }

private var _apple: ImageVector? = null
