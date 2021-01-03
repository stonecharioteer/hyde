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

Check out the documentation on [docs.rs/crate/hyde/0.1.0](https://docs.rs/crate/hyde/0.1.0).

## Usage

`hyde` supports the following commands:

### `hyde config`

This command can be used to create or update configurations. `hyde` stores its configs
in platform-dependent folders. To override this behaviour, set the `HYDE_CONFIG_FOLDER` path.
Note that hyde uses multiple config files, so a *folder* is required.

`hyde config create` will dump a default configuration file `default.toml` into the config folder. It takes the following optional parameters:

`hyde --name blog config create` will create `blog.toml` in the config folder.

`hyde -n website config create` will create `website.toml` in the config folder.

A config file looks like this:

```toml
root_folder = "/home/my_user_name/workspace/notes"

[drafts]
drafts = true
folder_name = "_drafts"
has_date = false

[drafts.default-tags]
categories = ["drafts", "notes"]

[published]
drafts = false
has_date = true
folder_name = "_posts"

[special]
drafts = false
has_date = false
folder_name = "pages
```

To update a field, use `hyde config set published.folder_name='something'`

### `hyde -n blog create 'A New Article on Writing'

### `hyde -n blog publish -d published a-new-article-on-writing

### `hyde -n website move -f published -t special a-new-article-on-writing

# Bug Reporting

While I developed `hyde` mostly for myself, anyone can file bug reports on [github](https://github.com/stonecharioteer/hyde).
