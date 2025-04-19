# tars - the artifact resolution system

Tars (stylized "tars") is a modular build system written in Python.

## How it works

Build targets are configured through a package configuration (*.tars*) file. Each target specifies a transform which is linked with an external plugin to perform it.

For example, a built-in tars plugin is `fs` (filesystem), providing access to file operations. One of the transforms provided by the `fs` plugin is `copy`, which, as the name suggests, copies a file from a source to a destination.

Tars loads this configuration file and executes the transforms on each target in order, to produce the final output.

Because of its plugin-based architecure, tars can be configured to work with pretty much anything.

## Usage

Currently, tars is quite basic, and just executes plugins, but more is always being worked on. 

`tars [-h] [-v] <config>`

 - `config`: the name of the tars package configuration file to run
 - `-v`, `--verbose`: enable verbose output
 - `-h`, `--help`: shows this information

## The name

Okay, the acronym was an afterthought.

The name is actually related to the small mammal the *Tarsier*, which you can read more about [here](https://en.wikipedia.org/wiki/Tarsier).

It's also the name of one of the robots from the film [Interstellar](https://en.wikipedia.org/wiki/Interstellar_(film)).
