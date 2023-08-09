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

public val AppIcons.Trash: ImageVector
    get() {
        if (_trash != null) {
            return _trash!!
        }
        _trash = Builder(name = "Trash", defaultWidth = 48.0.dp, defaultHeight = 48.0.dp,
                viewportWidth = 256.0f, viewportHeight = 256.0f).apply {
            path(fill = SolidColor(Color(0xFF778f9b)), stroke = SolidColor(Color(0x00000000)),
                    strokeLineWidth = 1.0f, strokeLineCap = Butt, strokeLineJoin = Miter,
                    strokeLineMiter = 4.0f, pathFillType = NonZero) {
                moveTo(128.0f, 21.333f)
                curveToRelative(-17.578f, 0.0f, -32.0f, 14.422f, -32.0f, 32.0f)
                lineToRelative(-33.354f, 0.0f)
                curveToRelative(-0.475f, -0.081f, -0.956f, -0.12f, -1.437f, -0.115f)
                curveToRelative(-0.388f, 0.01f, -0.774f, 0.048f, -1.156f, 0.115f)
                lineToRelative(-20.052f, 0.0f)
                curveToRelative(-2.885f, -0.041f, -5.569f, 1.475f, -7.023f, 3.967f)
                curveToRelative(-1.454f, 2.492f, -1.454f, 5.574f, 0.0f, 8.066f)
                curveToRelative(1.454f, 2.492f, 4.138f, 4.008f, 7.023f, 3.967f)
                lineToRelative(13.333f, 0.0f)
                lineToRelative(0.0f, 136.0f)
                curveToRelative(0.0f, 16.105f, 13.228f, 29.333f, 29.333f, 29.333f)
                lineToRelative(90.667f, 0.0f)
                curveToRelative(16.105f, 0.0f, 29.333f, -13.228f, 29.333f, -29.333f)
                lineToRelative(0.0f, -136.0f)
                lineToRelative(13.333f, 0.0f)
                curveToRelative(2.885f, 0.041f, 5.569f, -1.475f, 7.023f, -3.967f)
                curveToRelative(1.454f, -2.492f, 1.454f, -5.574f, 0.0f, -8.066f)
                curveToRelative(-1.454f, -2.492f, -4.138f, -4.008f, -7.023f, -3.967f)
                lineToRelative(-20.021f, 0.0f)
                curveToRelative(-0.859f, -0.141f, -1.735f, -0.141f, -2.594f, 0.0f)
                lineToRelative(-33.385f, 0.0f)
                curveToRelative(0.0f, -17.578f, -14.422f, -32.0f, -32.0f, -32.0f)
                close()
                moveTo(128.0f, 37.333f)
                curveToRelative(8.929f, 0.0f, 16.0f, 7.071f, 16.0f, 16.0f)
                lineToRelative(-32.0f, 0.0f)
                curveToRelative(0.0f, -8.929f, 7.071f, -16.0f, 16.0f, -16.0f)
                close()
                moveTo(69.333f, 69.333f)
                lineToRelative(117.333f, 0.0f)
                lineToRelative(0.0f, 136.0f)
                curveToRelative(0.0f, 7.457f, -5.876f, 13.333f, -13.333f, 13.333f)
                lineToRelative(-90.667f, 0.0f)
                curveToRelative(-7.457f, 0.0f, -13.333f, -5.876f, -13.333f, -13.333f)
                close()
                moveTo(109.208f, 95.885f)
                curveToRelative(-4.414f, 0.069f, -7.938f, 3.7f, -7.875f, 8.115f)
                lineToRelative(0.0f, 80.0f)
                curveToRelative(-0.041f, 2.885f, 1.475f, 5.569f, 3.967f, 7.023f)
                curveToRelative(2.492f, 1.454f, 5.574f, 1.454f, 8.066f, 0.0f)
                curveToRelative(2.492f, -1.454f, 4.008f, -4.138f, 3.967f, -7.023f)
                lineToRelative(0.0f, -80.0f)
                curveToRelative(0.031f, -2.163f, -0.815f, -4.247f, -2.346f, -5.776f)
                curveToRelative(-1.531f, -1.529f, -3.616f, -2.373f, -5.779f, -2.339f)
                close()
                moveTo(146.542f, 95.885f)
                curveToRelative(-4.414f, 0.069f, -7.938f, 3.7f, -7.875f, 8.115f)
                lineToRelative(0.0f, 80.0f)
                curveToRelative(-0.041f, 2.885f, 1.475f, 5.569f, 3.967f, 7.023f)
                curveToRelative(2.492f, 1.454f, 5.574f, 1.454f, 8.066f, 0.0f)
                curveToRelative(2.492f, -1.454f, 4.008f, -4.138f, 3.967f, -7.023f)
                lineToRelative(0.0f, -80.0f)
                curveToRelative(0.031f, -2.163f, -0.815f, -4.247f, -2.346f, -5.776f)
                curveToRelative(-1.531f, -1.529f, -3.616f, -2.373f, -5.779f, -2.339f)
                close()
            }
        }
        .build()
        return _trash!!
    }

private var _trash: ImageVector? = null
