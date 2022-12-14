# Overview

This is a simple rust practice project. It implements the github identicon algorithm as published [here](https://github.com/dgraham/identicon). Part of the code is deliberately over-engineered for practice purpose.

# Usage

Tip: you may find your github id via this api: `https://api.github.com/users/<github name>`.

Below are some typical examples. Full usage is displayed by running with `-h` flag.

- To render the image (as png):
  ```sh
  $ identicon 21012146 render hubot.png 
  ```
- To encode the image as jpeg format and output its base64 encoding:
  ```sh
  $ identicon 21012146 encode jpeg
  ```

# About
Re-implement based on the original identicon port [here](https://github.com/dgraham/identicon).
