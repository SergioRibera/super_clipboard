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

public val AppIcons.Pin: ImageVector
    get() {
        if (_pin != null) {
            return _pin!!
        }
        _pin = Builder(name = "Pin", defaultWidth = 100.0.dp, defaultHeight = 100.0.dp,
                viewportWidth = 100.0f, viewportHeight = 100.0f).apply {
            path(fill = SolidColor(Color(0xFF778F9B)), stroke = SolidColor(Color(0x00000000)),
                    strokeLineWidth = 0.0f, strokeLineCap = Butt, strokeLineJoin = Miter,
                    strokeLineMiter = 4.0f, pathFillType = NonZero) {
                moveTo(40.004f, 9.012f)
                curveTo(45.288f, 11.21f, 42.721f, 14.088f, 40.844f, 17.383f)
                curveTo(46.001f, 22.521f, 51.187f, 27.872f, 56.631f, 32.947f)
                curveTo(57.628f, 33.875f, 59.661f, 33.765f, 61.237f, 33.984f)
                curveTo(66.411f, 34.703f, 71.765f, 34.752f, 76.734f, 36.167f)
                curveTo(84.772f, 38.456f, 88.224f, 46.265f, 80.727f, 52.837f)
                curveTo(76.874f, 56.216f, 73.329f, 59.946f, 69.297f, 63.853f)
                curveTo(75.986f, 70.521f, 82.16f, 76.779f, 88.464f, 82.905f)
                curveTo(90.696f, 85.073f, 92.523f, 87.683f, 90.096f, 90.103f)
                curveTo(87.68f, 92.512f, 85.068f, 90.712f, 82.905f, 88.463f)
                curveTo(76.81f, 82.129f, 70.578f, 75.928f, 63.878f, 69.146f)
                curveTo(59.669f, 73.528f, 55.938f, 77.523f, 52.085f, 81.396f)
                curveTo(46.515f, 86.993f, 40.334f, 85.711f, 36.868f, 78.605f)
                curveTo(34.162f, 73.058f, 33.959f, 67.267f, 33.862f, 61.295f)
                curveTo(33.826f, 59.124f, 32.698f, 56.542f, 31.226f, 54.904f)
                curveTo(26.889f, 50.076f, 22.133f, 45.626f, 17.531f, 41.039f)
                curveTo(17.306f, 40.815f, 16.977f, 40.694f, 16.768f, 41.721f)
                curveTo(14.551f, 41.992f, 12.333f, 42.263f, 10.116f, 42.533f)
                curveTo(10.137f, 40.457f, 9.404f, 37.952f, 10.301f, 36.373f)
                curveTo(16.691f, 25.119f, 26.091f, 16.734f, 36.838f, 9.766f)
                curveTo(37.602f, 9.271f, 38.673f, 9.248f, 40.004f, 9.012f)
                moveTo(37.487f, 24.991f)
                curveTo(36.309f, 23.911f, 35.13f, 22.832f, 33.81f, 21.623f)
                curveTo(29.304f, 25.701f, 25.143f, 29.467f, 21.032f, 33.188f)
                curveTo(26.904f, 39.528f, 32.67f, 45.478f, 38.061f, 51.749f)
                curveTo(39.6f, 53.54f, 40.759f, 56.234f, 40.864f, 58.569f)
                curveTo(41.15f, 64.931f, 40.688f, 71.342f, 44.111f, 76.095f)
                curveTo(54.93f, 65.526f, 65.769f, 54.937f, 76.625f, 44.331f)
                curveTo(71.364f, 41.659f, 65.86f, 39.912f, 59.482f, 41.679f)
                curveTo(57.71f, 42.171f, 54.873f, 40.967f, 53.298f, 39.63f)
                curveTo(48.007f, 35.137f, 43.066f, 30.234f, 37.487f, 24.991f)
                close()
            }
        }
        .build()
        return _pin!!
    }

private var _pin: ImageVector? = null
