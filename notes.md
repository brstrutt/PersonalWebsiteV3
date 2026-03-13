## Notes

A file to store notes about any technical challenges I had to overcome while working on this project.

## Github Pages and Leptos Router

Github Pages always hosts sites with a URL suffix by default `https://brstrutt.github.io/PersonalWebsiteV3/`.
Leptos router doesn't play very well with this, as it assumes the root of the site is just `https://brstrutt.github.io/`.

This introduces two irritating bugs:
- When first loaded the webpage will show the 404 page, because the `PersonalWebsiteV3` route isn't recognised by the Router.
- When navigating around the website, the URL will get mangled to something like `https://brstrutt.github.io/projects`. This page will show the github pages 404 if ever reloaded.

It is possible to add `base="/PersonalWebsiteV3"` property to the `Router` component, but this doesn't change the routing behaviour of the `<A>` component. So the URL still gets mangled. Now with the added downside of the local dev server showing a 404 due to the local server not applying the suffix out of the box.

I have resolved this issue by adding a `const URL_SUFFIX: &str = "/PersonalWebsiteV3";` constant, specifying this as the `base` in the `Router` component, and creating a custom `InternalLink` component that automatically prepends this suffix to all hrefs.
This combined with adding the following `Trunk.toml` should hopefully work.

```Trunk.toml
version = "0.1"

[build]
public_url = "/PersonalWebsiteV3/"
```