#!/usr/bin/python3

#########################################
# tars - the artifact resolution system #
# https://github.com/OldUser101/tars    # 
#########################################

import importlib.util
import colorama
import os, sys
import json
import glob

"""
Represents each tars target
"""
class TarsTarget:
    def __init__(self):
        self.name = ""
        self.type = ""
        self.transform = ""
        self.package = ""
        self.config = {}

    """
    Parse target configuration
    """
    def parse(self, j, p):
        try:
            TarsUtils.check_has("name", j, "target")
            self.name = str(j["name"])

            if "type" in j:
                self.type = str(j["type"])

            TarsUtils.check_has("transform", j, f"target {p}:{self.name}")
            self.transform = str(j["transform"])

            self.config = j
            self.package = p

        except Exception as e:
            TarsUtils.error(e)

    """
    Run the transform on this target
    """
    def run(self, tp):
        tp.run_transform(self.transform, self.config, f"{self.package}:{self.name}")

"""
Parse a tars configuration file (.tars), run targets
"""
class TarsPackage:
    def __init__(self):
        self.targets = []

    """
    Loads and parses the package configuration file
    """
    def parse_from_file(self, tars):
        try:
            config = TarsConfiguration.load_config(tars)

            TarsUtils.check_has("package", config, tars.config_file)
            self.pkg_name = str(config["package"])

            if "targets" in config:
                for t in config["targets"]:
                    tx = TarsTarget()
                    tx.parse(t, self.pkg_name)
                    self.targets.append(tx)

        except Exception as e:
            TarsUtils.error(e)

    """
    Run all loaded targets sequentially
    """
    def run_targets(self, tp):
        for t in self.targets:
            t.run(tp)

"""
Context class passed to tars plugins when being registered
This is filled with various data about the plugin
"""
class TarsPluginContext:
    def __init__(self, name, path):
        self.name = name
        self.path = path
        self.transforms = {}

    def register_transform(self, name, func):
        self.transforms[name] = func

"""
Loads, registers, and executes tars plugins
"""
class TarsPluginManager:
    def __init__(self, tars, pluginDir=None):
        self.tars = tars
        self.plugins = {}
        self.pluginDir = pluginDir or os.path.expanduser("~/.tars/plugins")
        self.transforms = {}

        try:
            if not os.path.exists(self.pluginDir):
                os.makedirs(self.pluginDir)
        except Exception as e:
            TarsUtils.error(e)

    """
    Load and register all plugins
    """
    def load_plugins(self):
        if not os.path.isdir(self.pluginDir):
            return
        
        for f in os.listdir(self.pluginDir):
            if not f.endswith(".py"):
                continue

            path = os.path.join(self.pluginDir, f)
            name = f[:-3]

            if self.tars.verbose:
                TarsUtils.info(f"loading plugin '{name}' ({path})")

            spec = importlib.util.spec_from_file_location(name, path)
            mod = importlib.util.module_from_spec(spec)

            try:
                spec.loader.exec_module(mod)

                if hasattr(mod, "register"):
                    ctx = TarsPluginContext(name, path)
                    mod.register(ctx)
                    self.plugins[name] = mod
                    resolved_transforms = {f"{name}:" + k: v for k, v in ctx.transforms.items()}
                    self.transforms.update(resolved_transforms)
                else:
                    TarsUtils.warn(f"plugin '{name}' ({path}) does not expose 'register'")

            except Exception as e:
                TarsUtils.error(f"'{e}' while loading plugin '{name}'")

    """
    Run a transform, given the configuration and context
    """
    def run_transform(self, transform, config, ctx):
        if transform in self.transforms:
            if self.tars.verbose:
                TarsUtils.info(f"running transform \033[93m'{transform}'\033[97m on \033[95m{ctx}")

            try:
                code = self.transforms[transform](config)
                if code != 0:
                    TarsUtils.info(f"transform \033[93m'{transform}'\033[97m returned code {code} on \033[95m{ctx}\033[97m")
                    sys.exit(code)
            except Exception as e:
                TarsUtils.error(f"'{e}' while running transform \033[93m'{transform}'\033[97m on \033[95m{ctx}")
        else:
            TarsUtils.error(f"cannot resolve transform \033[93m'{transform}'\033[97m on \033[95m{ctx}")

"""
Various utility functions used by tars
"""
class TarsUtils:
    def check_has(n, k, i):
        if not n in k:
            TarsUtils.error(f"\033[93m'{n}'\033[97m must be specified in \033[95m{i}")

    def error(msg, noexit=False):
        print(f"\033[91merror: \033[97m{msg}\033[0m")
        if not noexit:
            sys.exit(1)

    def warn(msg):
        print(f"\033[93mwarn: \033[97m{msg}\033[0m")

    def info(msg):
        print(f"\033[94minfo: \033[97m{msg}\033[0m")

"""
Represents tars command line arguments
"""
class TarsArgs:
    def __init__(self):
        self.config = ""
        self.verbose = False

class TarsConfiguration:      
    """
    Given the config argument, resolve the actual configuration file
    """
    def get_config(tars):
        tars_files = glob.glob("*.tars")
        config_file = ""

        target = tars.config
        if not target.endswith(".tars"):
            target += ".tars"

        for tf in tars_files:
            if tf == target:
                config_file = tf
                break

        if config_file == "":
            TarsUtils.error(f"cannot find config file '{sys.argv[1]}'", noexit=True)
            print("\n\033[97mthe following are available in this directory:")

            for tf in tars_files:
                print(f"\t- \033[95m{tf.replace(".tars", "")}\033[97m.tars")

            print("\033[0m")
            sys.exit(1)

        return config_file

    """
    Just load the configuration file as JSON
    """
    def load_config(tars):
        try:
            with open(tars.config_file, "r") as f:
                data = json.load(f)
                return data
        except Exception as e:
            TarsUtils.error(e)

    """
    Parse command line arguments into a TarsArgs structure
    """
    def parse_args(tars):
        ta = TarsArgs()

        for arg in sys.argv[1:]:
            if arg.startswith("-"):
                if arg == "-v" or arg == "--verbose":
                    ta.verbose = True
                elif arg == "--version":
                    tars.version()
                    sys.exit(0)
                elif arg == "-h" or arg == "--help":
                    tars.usage()
                    sys.exit(0)
                else:
                    tars.usage(nl=True)
                    TarsUtils.error(f"unknown argument '{arg}' provided")
                    sys.exit(1)
            elif ta.config == "":
                ta.config = arg
            else:
                tars.usage(nl=True)
                TarsUtils.error("too many arguments provided")
                sys.exit(1)

        if ta.config == "":
            tars.usage(nl=True)
            TarsUtils.error("argument <config> is required")
            sys.exit(1)

        return ta

class Tars:
    VERSION = "0.1.0"

    """
    Initialize, parse, and run a tars configuration
    """
    def __init__(self):
        colorama.init() # Initialize colorama for colored output on Windows 

        args = TarsConfiguration.parse_args(self)

        self.config = args.config
        self.verbose = args.verbose

        self.config_file = TarsConfiguration.get_config(self)

        pkg = TarsPackage()
        pkg.parse_from_file(self)

        tp = TarsPluginManager(self)
        tp.load_plugins()

        pkg.run_targets(tp)

    """
    Displays the tars help message
    """
    def usage(self, nl=False):
        print("\033[92musage:\033[97m tars [-h] [-v] [--version] <config>")
        print("\n\033[95mtars\033[97m - the artifact resolution system\n")
        print("required arguments:")
        print("\t\033[92mconfig\033[97m\tthe tars package configuration file (.tars) to process\n")
        print("optional arguments:")
        print("\t\033[92m-v, --verbose\033[97m\tenable verbose output")
        print("\t\033[92m-h, --help\033[97m\tdisplays this help message")
        print("\t\033[92m--version\033[97m\tshow tars version\033[0m")

        if nl:
            print()

    def version(self):
        print(f"\033[95mtars\033[97m version {self.VERSION}")
        print("(c) 2025, Nathan Gill. See LICENSE for details.")
        print("https://github.com/OldUser101/tars\033[0m")

if __name__ == "__main__":
    Tars()