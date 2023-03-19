https://user-images.githubusercontent.com/56278796/225697681-418086a9-a02e-4087-8588-09fef558eaa6.mp4

</br>
<p align="center">
	<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/super_clipboard/ci.yml?label=ci&style=flat-square">
	<img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/super_clipboard/build.yml?style=flat-square">
    <a href="https://github.com/SergioRibera/super_clipboard/releases"><img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/SergioRibera/super_clipboard?label=download&style=flat-square"></a>
</p>

### Inspiration
In search of a clipboard manager that is comfortable and I can have instantly next to the mouse to speed up my work and at the same time I can include in my workflow and my environment based on WM's, SuperClipboard was born.

> **Warning:** It can run in gnome, but details like mouse tracking, transparency and window colors don't work quite right, if you would like to work on gnome support you can leave your feedback in this [issue](https://github.com/SergioRibera/super_clipboard/issues/2) to take it into account.

### Features
- Clean UI
- Text and Image clipboard
- Big clipboard history
- Follow cursor
- Position based on the display of each screen
- Transparent UI

> **NOTE:** The blur it's depends of your compositor

### Usage
On your startup WM script call binary as another task

bspwm example
```
super_clipboard &
```

To show Logs you need set `SUPER_CLIPBOARD_LOG` env variable, here some examples
> **NOTE:** For more details see [docs](https://docs.rs/env_logger/latest/env_logger/#enabling-logging)
```bash
# Show only errors
SUPER_CLIPBOARD_LOG=error super_clipboard
# Dump to file
SUPER_CLIPBOARD_LOG=info super_clipboard >> /tmp/super_clipboard_logs.txt
# Show only super_clipboard crate logs
SUPER_CLIPBOARD_LOG=off,super_clipboard=info super_clipboard
# Show only super_clipboard::daemon crate logs
SUPER_CLIPBOARD_LOG=off,super_clipboard::daemon=info super_clipboard
```

### TODO
- [x] Fix keyboard shortcut initial parsing
- [x] Fix performance increase
- [x] Position based on the display of each screen
- [x] Fix Unnautorize remap shortcut
- [x] Fix auto hide and show window
- [ ] Autopaste when select from history

Icons by <a target="_blank" href="https://icons8.com">Icons8</a>
