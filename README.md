# Hyde - Publish Your Markdown Easily

Hyde helps you create and manage markdown files and their frontmatter content.
The people who can use Hyde are users of markdown-based static website generators,
or anyone who maintains notes using markdown. Hyde helps you manage the location of
your markdown files, and it also helps you update the [YAML frontmatter](https://jekyllrb.com/docs/front-matter/) with ease. You can also move your files from folder to folder within one of these markdown projects and change properties such as permalinks, or apply default
tags to articles.

## Features

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

### Configuration

Before you use Hyde, you need to configure it. Indeed, Hyde will not run until you do.

Run the following command

`hyde config --help`

This command can be used to create or update configurations. Hyde stores its configs
in platform-dependent folders. To override this behaviour, set the `HYDE_CONFIG_FOLDER` path.
Note that Hyde uses multiple config files, so a *folder* is **required**.

`hyde config create` will dump a default configuration file `default.toml` into the config folder. It takes the following optional parameters:

`hyde --name blog config create` will create `blog.toml` in the config folder.

`hyde -n website config create` will create `website.toml` in the config folder.

---
**Config File Structure**

A config file looks like this:

```toml
root_folder = "/home/my_user_name/workspace/notes"

[drafts]
drafts = true
folder_name = "_drafts"
has_date = false

[drafts.default-frontmatter]
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

At the very top of the file is the `root_folder` property. This dictates where your posts go.

There are multiple sections in the toml file. Their names do not matter, only their properties do.
There needs to be a block with `drafts = true`, and there needs to be one with `drafts = false`,
everything beyond that is specific to your needs. A draft folder stores posts *without*
timestamps in the frontmatter. If a block doesn't have a `folder_name` property, then, the block's
name will be used as the folder name.

---

To update a field, use `hyde config set published.folder_name='something'`

### Creating a New Draft Post

The `create` subcommand can be used to create a new draft.

Consider: `hyde -n blog create 'A New Article on Writing'`. This will create a new file
`a-new-article-on-writing.md` in the first folder (within the `blog` root folder) which is marked with `drafts = true` in your
config. If you have multiple draft folders and would like to control which is the default, set
`default = true` in that block.

The newly created post will use the `default-frontmatter` properties from the config block,
and it will not have a timestamp. By default, `draft` blocks will have `has_date = false`.

### Publish a Post

To publish a post, use the `publish` subcommand.

Example: `hyde -n blog publish 'A New Article on Writing'`. This command will *move* an article
that contains 'A New Article on Writing' as a title. Note that this command supports regex
strings, if you provide them. The string is evaluated against the file names, *and* the title
within the frontmatter. [The syntax is PCRE, but without lookarounds or backreferences.](https://github.com/rust-lang/regex)

### Move a Post

You can also move a post between published folders, or between draft folders. **Note that you cannot move posts from a draft folder to a published folder.** Use the `publish` command to do so.

Example: `hyde -n website move -f published -t special a-new-article-on-writing` moves an article matching `a-new-article-on-writing` from the `published` folder (again, using the name of the block, but the actual folder name depends on the `folder_name` property if it exists) to the `special` folder.

## Bug Reporting

While I developed `hyde` mostly for myself, anyone can file bug reports on [github](https://github.com/stonecharioteer/hyde).
