# `hyde` - Markdown Template Tool

Create and Manage Markdown files and their header content.

Hyde is a tool that helps you make Jekyll-based Markdown files.

Hyde can:

    1. Create a new post

    2. Create a new draft

    3. Move draft to post and add the relevant date.

    4. Add metadata to existing post.

    5. Modify title block metadata for a post.

Hyde also implements some checks and balances:

Hyde will warn you when:

    1. A post of the same name has been made before

    2. A post of the same permalink has been made before.

Hyde uses `toml` files for configuration files. It stores its config
in `$HOME/.config/hyde/` in Linux and OSX,
and in the `%APPDIR%` on Windows.

Check out the documentation on [docs.rs/hyde](https://docs.rs/hyde).
