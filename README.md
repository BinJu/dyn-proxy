# Remove div tags from the HTML.
## How it Works.
This program listens at the given port, and wait for the access from your browser. If it is
received from your browser, it will then send the request to the base url plus the uri that
is from your browser. The response from the destination will be filtered by remving some of the
\<div\> tag that you specified in the command line. Multiple div properties could be applied,
thus you can remove multiple \<div\> tags. For example:
```
dyn-proxy 'https://www.merriam-webster.com' 3000 'id="definition-right-rail"' 'class="border-box mobile-fixed-ad"' 'class="abl mw-ad-slot-top"'
```

## How to Run.
```
dyn-proxy <base-url> <listening port> <div properties>{1..n}
```
The div properties should not include the div tag. for example, if you want to filter out the tag `<div id="user-name">`, the property would be `'id="user-name"'`.
The double quote for user-name can not be omitted.

## How to extend.
**TODO**

## License
MIT
