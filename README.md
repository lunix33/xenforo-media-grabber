# You probably don't want to use this project.

This project was made to quickly grab the content of a XF lightbox gallery.

It was made for a specific website and may not work for other lightbox gallery
if the HTML layout is different. Also It will not work on a newer version of XF
since I'm fairely certain website was using XF1, not XF2 or newer.

Use at your own risks. I won't fix any bug, nor will I add any new features.

```
Usage: xenforo-gallery-grabber [OPTIONS] --origin <ORIGIN> --path <PATH> --out <OUT>

Options:
      --origin <ORIGIN>    The base URL of the website. Example: https://my-website.com/
      --path <PATH>        The path of the gallery within the website, uses `{}` as a replacement pattern for the page number. Example: media/?page={}
      --start <START>      The first page to grab.
      --end <END>          The last page to grab.
      --out <OUT>          The output directory for the downloaded files.
      --session <SESSION>  The session cookie used by your browser. You'll need to use the dev tools of your browser and grab your session cookie. Example: xf_session=f862807a987b2aa197e1fc208f75cc30
      --rate <RATE>        The number of ms between requests. Useful if the website doesn't allow quick requests.
  -h, --help               Print help information
```
