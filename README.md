# gh-auth

**gh-auth** is a simple utility that serves files to users authenticated from
GitHub.

To use it, you will need to register a new OAuth Application. More information
can be found on [GitHub's Documentation](https://docs.github.com/en/developers/apps/creating-an-oauth-app).

Then, start `gh-auth` after setting the following environment variables:

- `GITHUB_KEY`: Your OAuth App's key
- `GITHUB_SECRET`: Your OAuth App's secret
- `SECRET_KEY`: A random string used to encrypt cookies
- `ALLOWED_USERS`: A comma-separated list of users allowed to authenticate on this instance.

With the following command-line options:

- `-host [0.0.0.0]`: Host to bind the server. Defaults to `0.0.0.0`.
- `-port [8000]`: Port to bind the server. Defaults to `8000`.
- `-root [PATH]`: Root path from where to serve files from.

## License

```
MIT License

Copyright © 2021 Victor Gama

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
