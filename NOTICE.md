# Legacy installer retirement

This branch preserves these historical Aderyn installer URLs:

- `https://raw.githubusercontent.com/Cyfrin/aderyn/master/cyfrinup/cyfrinup`
- `https://raw.githubusercontent.com/Cyfrin/aderyn/master/cyfrinup/dynamic_script`

The original setup URL, `https://raw.githubusercontent.com/Cyfrin/aderyn/dev/cyfrinup/install`,
is preserved separately on the `dev` branch with the same retirement behavior.

Older installed clients fetch these paths and pipe their contents into a shell.
Aderyn is retired, so each endpoint returns valid shell that prints a retirement
notice to stderr and exits with status 1. It does not download software, modify
existing installations, or forward to the shared Cyfrin/Up installer.

Keep the `master` branch and both `cyfrinup` paths available. Do not
delete or rename them, replace the script with HTML, or restore installation
logic. Keep this branch limited to legacy compatibility files and this notice.

The shared [Cyfrin/Up](https://github.com/Cyfrin/Up) installer remains separate
and continues serving maintained Cyfrin tools.
