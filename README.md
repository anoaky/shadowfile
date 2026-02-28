# Shadowfile

A library for mocking ("shadowing") files in memory from embedded data, allowing other dependencies to access embedded data as a file.

It was originally created to circumvent `gettext`'s file requirements, but it can be used in any situation where you want to
trick a dependency into thinking some embedded data is a file. Potentially useful in environments with limited/slow storage.

## Uses

- Unit tests
- One of your dependencies requires files as input (e.g., `gettext`)

## License

This project is licensed under the MIT or Apache v2 license. 

Parts of this project are licensed under the Unicode v3 license, which can be found in [licenses/unicode.txt](licenses/unicode.txt).