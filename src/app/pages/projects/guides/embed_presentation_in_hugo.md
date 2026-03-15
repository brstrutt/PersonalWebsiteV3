---
title: "Embed a presentation in a hugo website"
date: 2024-08-18
lastmod: 2024-08-18
tags:
- hugo
- presentation
- reveal.js
---

## Why?

I suddenly find myself needing a way to do a presentation. But I'm on linux, and don't want to mess about with libre office vs powerpoint.
But I have this website. Surely there's an easy way to just...host the presentation on the site? Then I don't need to distribute a disgusting open-office alternative to a .ppx.

But how to do this?

## Use an existing hugo theme

It turns out somebody has already thought of this and produced [this hugo theme](https://github.com/joshed-io/reveal-hugo). My website is already hugo so it should just drop in right?
Then I'll be able to write presentations in MARKDOWN. Hell yes. Nice and easy.

Following their instructions is quite easy. Download up their theme, use their theme, put the following snippet in your hugo.toml and away you go!

```toml
[outputFormats.Reveal]
baseName = "index"
mediaType = "text/html"
isHTML = true
```

Then just create a presentation by putting this in the [hugo frontmatter](https://gohugo.io/content-management/front-matter/) of the page that you want to be a presentation:

```toml
outputs = ["Reveal"]
```

## Problems

This theme seems to assume you want the ENTIRE site to be a single presentation. This is not what I want.
On top of this I want the rest of my site to keep using MY theme, and just have specific presentation pages use this one. How the fuck?

To use [multiple themes](https://gohugo.io/hugo-modules/theme-components/#readout) on your hugo site you specify the themes in the `hugo.toml` file like this:

```toml
theme = ['custom', 'reveal-hugo']
```

From a glance at the documentation I thiiiiink this squishes the two themes together into one theme. Anytime both themes have a file the file on the leftmost theme will be used.

This took some messing with to get it to work, but in the end I just needed to modify the `reveal-hugo` theme to:

- delete the `list.html` and `single.html` files.
- copy the contents of `index.reveal.html` into `single.reveal.html`
- add a `_markup/render-heading.reveal.html` to override the custom rendering I have in my custom theme (it was causing issues)

## Result

Basic left->right presentations now work. Easily definable in a single markdown file.
I still need to figure out how to link back to the main website from a presentation page, but that's a problem for later.
