# SimpleAst

A port of [discordapp/simpleast](https://github.com/discordapp/SimpleAST) to Rust.

A library to create simple ast parsers using regex based rules.

Originally for Discord-flavored markdown, intended for use in
[discord clients](https://github.com/terminal-discord/weechat-discord)

The api is generic and should allow custom ast types and rules.

By default this library uses oniguruma for regex, however this can be changed
by disabling the default features and enabling the "pcre" flag which will use
the pcre2 library.