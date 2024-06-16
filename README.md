# Environment File
Create a `.env` file with the following variables.

* `DATABASE_URL`: a diesel-recognized URL to a postgres database
* `CERT_PATH`: path to a public SSL certificate[^1]
* `KEY_PATH`: path to the key associated with the above

[^1]: for local development you can make one with `mkcert`.