# Surrogate
![alt text](logo.png)

![Rust CI](https://github.com/vmanikes/Surrogate/actions/workflows/rust.yml/badge.svg)
![DevSkim](https://github.com/vmanikes/Surrogate/actions/workflows/devskim.yml/badge.svg)
![Clippy](https://github.com/vmanikes/Surrogate/actions/workflows/rust-clippy.yml/badge.svg)

Surrogate is a templating engine and acts like a swiss army knife for all your templating needs.

## Installation
- Make sure you have `rust` and `cargo` installed
- Run the following command to build the surrogate binary `cargo build --release` and you can see the binary listed in the 
`target` directory
- Move the built binary to `/usr/bin`

## Usage
```text
surrogate --help

   _____                                  _
  / ____|                                | |
 | (___  _   _ _ __ _ __ ___   __ _  __ _| |_ ___
  \___ \| | | | '__| '__/ _ \ / _` |/ _` | __/ _ \
  ____) | |_| | |  | | | (_) | (_| | (_| | ||  __/
 |_____/ \__,_|_|  |_|  \___/ \__, |\__,_|\__\___|
                               __/ |
                              |___/

Terraform backend and version templater

Usage: surrogate [OPTIONS]

Options:
  -d, --directory <DIRECTORY>  Directory against which surrogate should run [default: .]
  -h, --help                   Print help information
  -V, --version                Print version information

```

## What problems does it solve?
Consider this example where the following is defined in your `main.tf` file

```terraform
terraform {
  backend "s3" {
    bucket         = "some-s3-bucket"
    key            = "sometestkey"
    region         = "us-east-1"
  }
}
```
Let's say you want to make the `region` dynamic and wanted to use a variable that is defined in `variables.tf`
```terraform
terraform {
  backend "s3" {
    bucket         = "some-s3-bucket"
    key            = "sometestkey"
    region         = var.region
  }
}
```
Now when you run `terraform apply`, you will get the error `Error: Variables not allowed`. This is really annoying and 
is the only place where things `needs` to be hardcoded.

Let's see how surrogate can help
- Let's create a file called `main.tf.tpl` in the same place where `main.tf` is defined.
- Copy the contents of `main.tf` to `main.tf.tpl` and change the fields that need to be dynamic in the following way
```
terraform {
  backend "s3" {
    bucket         = "some-s3-bucket"
    key            = "sometestkey"
    region         = {{ region }}
  }
}
```
- Create a file in the root of your project called `Surrogate.json` with the following contents
```json
{
  "region": "us-east-1"
}
```
- You can now go ahead and delete the `main.tf` file and run the surrogate cli to see the magic
```shell
surrogate -d {path to the project}
```
- Surrogate now will look for all the file with `.tpl` suffix and replace all the occurrences of `{{region}}` with `us-east-1`
and will create a new file called `main.tf` with contents replaced
- Cool ain't it eh !!!
