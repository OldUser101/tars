# tars - the artifact resolution system

Tars (stylized "tars") is a modular build system written in Python.

## How it works

At its core, tars is incredibly simple, it parses a configuration file, and dispatches targets to relevant transform plugins. It is a single, ~300 line Python file at the time of writing.

Build targets are configured through a package configuration (*.tars*) file. Each target specifies a transform which is linked with an external plugin to perform it.

For example, a built-in tars plugin is `fs` (filesystem), providing access to file operations. One of the transforms provided by the `fs` plugin is `copy`, which, as the name suggests, copies a file from a source to a destination.

```json
{
    "package": "tars-test",
    "targets": [
        {
            "name": "hello-copy",
            "transform": "fs:copy",
            "src": "src/hello.txt",
            "dest": "dst/"
        }
    ]
}
```

Tars loads this configuration file and executes the transforms on each target in order, to produce the final output.

Because of its plugin-based architecure, tars can be configured to work with pretty much anything.

## Installing

Installing tars on your system is very simple.

Prerequisites:
 - Python 3 and default modules (most versions should work, but tested on Python 3.13)
 - Python `colorama` package (`pip install colorama` or similar)

### Method 1: Script

Run this to download the install script and run it:

`wget -O ~/tars-install.sh https://raw.githubusercontent.com/OldUser101/tars/refs/heads/main/install.sh && chmod +x ~/tars-install.sh && ~/tars-install.sh && rm ~/tars-install.sh`

This will install tars for the current user.

### Method 2: Manual

Simply clone this repository with:

`git clone https://github.com/OldUser101/tars.git`

to any suitable location of your choice.

Then symlink `tars` to `tars.py` with:

`ln -s /usr/bin/tars /path/to/where/you/cloned/tars.py`

You don't have to put this symlink in `/usr/bin`, though, if you do, you should probably copy `tars.py` to a location accessible by all users if you're on a multi-user system.

From there, just type `tars` to run it.

I'm currently working on tars standard plugins [over here](https://github.com/OldUser101/tars-plugins).

## Usage

Currently, tars is quite basic, and just executes plugins, but more is always being worked on. 

`tars [-h] [-v] <config>`

 - `config`: the name of the tars package configuration file to run
 - `-v`, `--verbose`: enable verbose output
 - `-h`, `--help`: shows this information

An example configuration looks like the following:

```json
{
    "package": "tars-test",
    "targets": [
        {
            "name": "hello-copy",
            "transform": "fs:copy",
            "src": "src/hello.txt",
            "dest": "dst/"
        }
    ]
}
```

This will run the `fs:copy` transform, copying `src/hello.txt` to the `dst` directory.

Targets can contain many values, this is defined by the transform being applied. The only two required values are `name` and `transform` which are used by tars. `type` is also reserved for use by tars.

Transforms are typically specified in the format `plugin:transform`, so `fs:copy` would be the `copy` transform in the `fs` plugin.

Hopefully you can see how easy tars is to use.

## Plugins

I'm still working on actually creating these plugins (even `fs` is just limited to the `copy` transform, and not even public), but you can always write your own.

All plugins should have a top-level function `register` that takes a `TarsPluginContext` object as an argument. In this function, plugins should register any transforms using the `register_transform` function.

Here is an example (`fs:copy`):

```py
import os, shutil

def fs_copy(cfg):
    if not "src" in cfg or not "dest" in cfg:
        return 1

    src = cfg["src"]
    dest = cfg["dest"]

    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copy2(src, dest)

    return 0

def register(ctx):
    ctx.register_transform("copy", fs_copy)
```

As you can probably see, `register` just calls `register_transform` with the transform name (`copy`), and the function to call (`fs_copy`).

It's worth noting that the transform names passed to `register_transform` should be the local name (e.g. `copy`, not `fs:copy`), as the plugin name is added by tars at runtime based on the name of the main plugin file. In this case, the file is `fs.py`, the transform is `copy`, so the full name would be `fs:copy`.

Transform functions take a single argument (`cfg` in this case), that is the configuration dictionary taken straight from the package configuration file. For example:

```json
{
    "name": "hello-copy",
    "transform": "fs:copy",
    "src": "src/hello.txt",
    "dest": "dst/"
}
```

Because the configuration is editable directly by the user, it is best to check if the `src` and `dest` keys actually exist in the configuration to avoid triggering exceptions.

We then make the directories and copy the file as required.

The last, and probably most important, line in the transform function is `return 0`. This signals tars that the transform ran successfully. If you don't return anything, or return a number that is not 0, tars will halt the entire build process, assuming something went wrong. **Please return something.**

It's worth noting that transforms can take any configuration they wish, as long as it's valid JSON, you can use it.

For example, you could have:

```json
{
    "name": "compile-hello",
    "transform": "c:compile",
    "src": [ "src/hello.c" ],
    "options": {
        "debug": true
    },
    "artifact": {
        "path": "bin/hello",
        "type": "binary"
    }
}
```

For the (currently fictional) plugin `c`, the `compile` transform could take various configuration options, such as multiple sources, options, and build artifacts. The only two required values are `name` and `transform`, as these are used by tars itself.

The target configuration value `type` is reserved for future use by tars, and plugins should not put their own things there.

Plugins are not limited to just the capabilities of Python either. With appropriate APIs, the Python plugin can be used as a wrapper to run other things on the system. For example, if you had written a plugin in Rust, you could run the binary with generated parameters. You could even load a shared library (or dynamic-linked library on Windows), and call functions from it. This allows tars to be extensible, and is the very reason it is so powerful.

## The name

Okay, the acronym was an afterthought.

The name is actually related to the small mammal the *Tarsier*, which you can read more about [here](https://en.wikipedia.org/wiki/Tarsier).

It's also the name of one of the robots from the film [Interstellar](https://en.wikipedia.org/wiki/Interstellar_(film)).