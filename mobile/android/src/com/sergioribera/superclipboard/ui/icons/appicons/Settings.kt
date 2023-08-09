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

public val AppIcons.Settings: ImageVector
    get() {
        if (_settings != null) {
            return _settings!!
        }
        _settings = Builder(name = "Settings", defaultWidth = 48.0.dp, defaultHeight = 48.0.dp,
                viewportWidth = 256.0f, viewportHeight = 256.0f).apply {
            path(fill = SolidColor(Color(0xFF778f9b)), stroke = SolidColor(Color(0xFF778f9b)),
                    strokeLineWidth = 1.0f, strokeLineCap = Butt, strokeLineJoin = Miter,
                    strokeLineMiter = 4.0f, pathFillType = NonZero) {
                moveTo(128.0f, 21.333f)
                curveToRelative(-8.41f, 0.0f, -16.519f, 1.064f, -24.26f, 2.865f)
                curveToRelative(-3.308f, 0.772f, -5.768f, 3.55f, -6.135f, 6.927f)
                lineToRelative(-1.698f, 15.479f)
                curveToRelative(-0.555f, 5.072f, -3.485f, 9.56f, -7.906f, 12.115f)
                curveToRelative(-4.412f, 2.549f, -9.767f, 2.831f, -14.437f, 0.781f)
                lineToRelative(-0.01f, 0.0f)
                lineToRelative(-14.219f, -6.26f)
                curveToRelative(-3.108f, -1.368f, -6.741f, -0.625f, -9.063f, 1.854f)
                curveToRelative(-11.053f, 11.784f, -19.537f, 26.053f, -24.354f, 41.979f)
                curveToRelative(-0.983f, 3.249f, 0.19f, 6.764f, 2.927f, 8.771f)
                lineToRelative(12.615f, 9.25f)
                curveToRelative(4.119f, 3.028f, 6.542f, 7.801f, 6.542f, 12.906f)
                curveToRelative(0.0f, 5.108f, -2.422f, 9.884f, -6.542f, 12.906f)
                lineToRelative(-12.615f, 9.24f)
                curveToRelative(-2.737f, 2.007f, -3.911f, 5.522f, -2.927f, 8.771f)
                curveToRelative(4.817f, 15.924f, 13.294f, 30.204f, 24.354f, 41.99f)
                curveToRelative(2.324f, 2.475f, 5.956f, 3.214f, 9.063f, 1.844f)
                lineToRelative(14.219f, -6.26f)
                curveToRelative(4.672f, -2.055f, 10.033f, -1.759f, 14.448f, 0.792f)
                curveToRelative(4.421f, 2.554f, 7.351f, 7.042f, 7.906f, 12.115f)
                lineToRelative(1.698f, 15.479f)
                curveToRelative(0.37f, 3.371f, 2.824f, 6.142f, 6.125f, 6.917f)
                curveToRelative(7.745f, 1.808f, 15.861f, 2.875f, 24.271f, 2.875f)
                curveToRelative(8.41f, 0.0f, 16.519f, -1.064f, 24.26f, -2.865f)
                curveToRelative(3.308f, -0.772f, 5.768f, -3.55f, 6.135f, -6.927f)
                lineToRelative(1.698f, -15.479f)
                curveToRelative(0.555f, -5.072f, 3.485f, -9.56f, 7.906f, -12.115f)
                curveToRelative(4.412f, -2.549f, 9.767f, -2.842f, 14.437f, -0.792f)
                lineToRelative(14.229f, 6.26f)
                curveToRelative(3.106f, 1.37f, 6.739f, 0.631f, 9.063f, -1.844f)
                curveToRelative(11.053f, -11.784f, 19.537f, -26.064f, 24.354f, -41.99f)
                curveToRelative(0.983f, -3.249f, -0.19f, -6.764f, -2.927f, -8.771f)
                lineToRelative(-12.615f, -9.24f)
                curveToRelative(-4.119f, -3.022f, -6.542f, -7.798f, -6.542f, -12.906f)
                curveToRelative(0.0f, -5.108f, 2.422f, -9.884f, 6.542f, -12.906f)
                lineToRelative(12.615f, -9.24f)
                curveToRelative(2.737f, -2.007f, 3.911f, -5.522f, 2.927f, -8.771f)
                curveToRelative(-4.818f, -15.926f, -13.301f, -30.206f, -24.354f, -41.99f)
                curveToRelative(-2.324f, -2.475f, -5.956f, -3.214f, -9.063f, -1.844f)
                lineToRelative(-14.229f, 6.26f)
                curveToRelative(-4.67f, 2.05f, -10.025f, 1.757f, -14.437f, -0.792f)
                curveToRelative(-4.421f, -2.554f, -7.351f, -7.042f, -7.906f, -12.115f)
                lineToRelative(-1.698f, -15.479f)
                curveToRelative(-0.37f, -3.371f, -2.824f, -6.142f, -6.125f, -6.917f)
                curveToRelative(-7.745f, -1.808f, -15.861f, -2.875f, -24.271f, -2.875f)
                close()
                moveTo(128.0f, 37.333f)
                curveToRelative(5.196f, 0.0f, 10.186f, 0.933f, 15.188f, 1.812f)
                lineToRelative(1.0f, 9.198f)
                curveToRelative(1.109f, 10.128f, 6.996f, 19.135f, 15.812f, 24.229f)
                curveToRelative(8.822f, 5.097f, 19.563f, 5.683f, 28.885f, 1.583f)
                lineToRelative(8.458f, -3.719f)
                curveToRelative(6.495f, 7.799f, 11.637f, 16.609f, 15.229f, 26.26f)
                lineToRelative(-7.5f, 5.5f)
                curveToRelative(-8.211f, 6.024f, -13.073f, 15.614f, -13.073f, 25.802f)
                curveToRelative(0.0f, 10.188f, 4.862f, 19.778f, 13.073f, 25.802f)
                lineToRelative(7.5f, 5.5f)
                curveToRelative(-3.592f, 9.652f, -8.734f, 18.461f, -15.229f, 26.26f)
                lineToRelative(-8.458f, -3.719f)
                curveToRelative(-9.322f, -4.1f, -20.063f, -3.514f, -28.885f, 1.583f)
                curveToRelative(-8.816f, 5.094f, -14.704f, 14.102f, -15.812f, 24.229f)
                lineToRelative(-1.0f, 9.198f)
                curveToRelative(-5.001f, 0.877f, -9.994f, 1.812f, -15.188f, 1.812f)
                curveToRelative(-5.196f, 0.0f, -10.186f, -0.933f, -15.188f, -1.812f)
                lineToRelative(-1.0f, -9.198f)
                curveToRelative(-1.109f, -10.128f, -6.996f, -19.135f, -15.812f, -24.229f)
                curveToRelative(-8.822f, -5.097f, -19.563f, -5.683f, -28.885f, -1.583f)
                lineToRelative(-8.458f, 3.719f)
                curveToRelative(-6.496f, -7.798f, -11.638f, -16.608f, -15.229f, -26.26f)
                lineToRelative(7.5f, -5.5f)
                curveToRelative(8.211f, -6.024f, 13.073f, -15.614f, 13.073f, -25.802f)
                curveToRelative(0.0f, -10.188f, -4.865f, -19.785f, -13.073f, -25.812f)
                lineToRelative(-7.5f, -5.5f)
                curveToRelative(3.593f, -9.655f, 8.741f, -18.459f, 15.24f, -26.26f)
                lineToRelative(8.448f, 3.719f)
                curveToRelative(9.322f, 4.1f, 20.063f, 3.524f, 28.885f, -1.573f)
                curveToRelative(8.816f, -5.094f, 14.704f, -14.102f, 15.812f, -24.229f)
                lineToRelative(1.0f, -9.198f)
                curveToRelative(5.001f, -0.877f, 9.994f, -1.812f, 15.188f, -1.812f)
                close()
                moveTo(128.0f, 85.333f)
                curveToRelative(-23.469f, 0.0f, -42.667f, 19.197f, -42.667f, 42.667f)
                curveToRelative(0.0f, 23.469f, 19.197f, 42.667f, 42.667f, 42.667f)
                curveToRelative(23.469f, 0.0f, 42.667f, -19.197f, 42.667f, -42.667f)
                curveToRelative(0.0f, -23.469f, -19.197f, -42.667f, -42.667f, -42.667f)
                close()
                moveTo(128.0f, 101.333f)
                curveToRelative(14.822f, 0.0f, 26.667f, 11.844f, 26.667f, 26.667f)
                curveToRelative(0.0f, 14.822f, -11.844f, 26.667f, -26.667f, 26.667f)
                curveToRelative(-14.822f, 0.0f, -26.667f, -11.844f, -26.667f, -26.667f)
                curveToRelative(0.0f, -14.822f, 11.844f, -26.667f, 26.667f, -26.667f)
                close()
            }
        }
        .build()
        return _settings!!
    }

private var _settings: ImageVector? = null
