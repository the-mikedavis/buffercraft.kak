# buffercraft.kak

Buffercraft is a plugin for the [kakoune](https://kakoune.org/)
text editor which tries to emulate some behavior of
[`tpope/vim-projectionist`](https://github.com/tpope/vim-projectionist).

## What is it?

Buffercraft is a plugin that allows you to jump around and render templates
for files in projects with common structures.

For example, Elixir has a
convention where any source module `MyApp.MyModule` is defined in a
`lib/my_app/my_module.ex` file. That module also has an _alternate_: the
unit test module/file. `MyApp.MyModule` is usually tested in a module
named `MyApp.MyModuleTest` located in `test/my_app/my_module_test.exs`.

If you're editing `MyApp.MyModule`, you'll usually make some
accompanying edits to the unit tests, and this means you have to
go open or jump to the buffer for the unit test file. This is dead
easy in pretty much every editor but it takes way too many keystrokes
(sometimes you type the whole filename! :scream:).

With buffercraft, you just use the `buffercraft-alternate` command to jump
from the source to the unit tests or from the unit tests back to the source.
This can be tab-completed with `<esc>:alt<tab><ret>`, or you can make a
custom mapping to really save on keystrokes!

If you're writing new source and test modules, you'll also have to type
a bit of boilerplate: the `defmodule/2` declarations. With the proper
template configuration, the `buffercraft-template` command can start
off a source file with the boilerplate

```elixir
defmodule MyApp.MyModule do
  @moduledoc """
  """
end
```

And once you jump to the unit tests, you can set up the boilerplate there
too:

```elixir
defmodule MyApp.MyModuleTest do
  use ExUnit.Case, async: true

  alias MyApp.MyModule
end
```

This isn't a huge time savings but once you start using these jumping and
templating abilities, doing it by hand will seem absurd and monotonous.

## Installation

If you use [nix](https://nixos.org/) for package management, you may
install the `kak-buffercraft` binary by cloning this repository and
running `nix-env -i -f default.nix`.

If you use flakes, you can add this flake to your flake as an input.
For example, a NixOS configuration can add the `kak-buffercraft` binary
like so

```nix
{
  inputs = {
    # ..
    kak-buffercraft.url = "github:the-mikedavis/buffercraft.kak";
  };
  outputs = inputs@{ pkgs, ... }: {
    nixosConfigurations.mymachinenamehere = pkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        # ..
        {
          environment.systemPackages = [
            inputs.kak-buffercraft.defaultPackage.x86_64-linux
          ];
        }
      ];
    };
  };
}
```

If you're just using `cargo` to build the rust part of this project,

```
cargo install
```

will install the `kak-buffercraft` binary into your cargo binaries path.

With the `kak-buffercraft` binary installed, now add the kakoune plugin.
You can drop in the `rc/buffercraft.kak` into your kakoune auto-load
directory ([see the kakoune wiki on plugin
installation](https://github.com/mawww/kakoune/wiki/Installing-Plugins)),
or if you use [`plug.kak`](https://github.com/andreyorst/plug.kak),
add

```kak
plug "the-mikedavis/buffercraft.kak"
```

to your kakoune configuration. This will add the `buffercraft-alternate` and
`buffercraft-template` commands which can be mapped or called directly.

## Configuration

Buffercraft defines four options for getting to alternates and rendering
templates. These should be configured in your own kakoune configuration
using hooks for buffer creation. For example, here are some of mine for
basic elixir projects.

```kak
hook global BufCreate .*[.]ex %{
  set-option buffer buffercraft_kind "lib"
  set-option buffer buffercraft_pattern "lib/(.*)\.ex"
  set-option buffer buffercraft_alternate "test/{{ matches[1] }}_test.exs"
  set-option buffer buffercraft_template \
'defmodule {{ matches[1] | pascalcase | dot }} do
  @moduledoc """
  """
end'
}

hook global BufCreate .*_test[.]exs %{
  set-option buffer buffercraft_kind "test"
  set-option buffer buffercraft_pattern "test/(.*)_test\.exs"
  set-option buffer buffercraft_alternate "lib/{{ matches[1] }}.ex"
  set-option buffer buffercraft_template \
'defmodule {{ matches[1] | pascalcase | dot }}Test do
  use ExUnit.Case, async: true

  alias {{ matches[1] | pascalcase | dot }}
end'
}
```

## Usage

Once these options are set up, edit a buffer matching the BufCreate regex
and use the `buffercraft-alternate` and `buffercraft-template` commands.
`buffercraft-alternate` edits a buffer using the `buffercraft_alternate`
as the template, and `buffercraft-template` renders the `buffercraft_template`
option's template in the current buffer.

## Development

This repo uses [nix](https://nixos.org/) for development. You may also
ignore nix and use `cargo` if you have it installed. Spawn a development
shell with all necessary dependencies with `nix shell`.

## How this works

### The `kak-buffercraft` binary

The `kak-buffercraft` file takes three inputs:

- template: a [Tera](https://github.com/Keats/tera) template passed via `stdin`
- pattern: the first command-line argument which is a regex pattern to match
  against the second argument (prospect)
- prospect: the second command-line argument which hydrates values to use in the template

For example, let's look at a common use-case in an elixir-lang project. A
project may have a `MyApp.MyModule` module defined in `lib/my_app/my_module.ex`.
This naming pattern is very consistent across elixir projects: source code
modules generally follow a template where `lib/(.*)\.ex` yields a single module
that initially looks like this:

```elixir
defmodule {{ matches[1] | pascalcase | dot }} do
  @moduledoc """
  """
end
```

Where `matches[1]` is the regex match for the first group (thing in parens).

This code block would be the _template_ (passed via `stdin`), while
`lib/(.*)\.ex` would be the _pattern_ (first command-line argument)
and `lib/my_app/my_module.ex` would be the _prospect_ (second command-line
argument).

What about alternates? Alternates may be used differently but the structure
is the same. The alternate for an elixir lib file has the same prospect
and pattern but uses a template:

```
test/{{ matches[1] }}_test.exs
```

And of course that template gets used differently by the `buffercraft.kak`
plugin: the alternate is used to jump around files and the template is used
to fill in a blank buffer.

### The `buffercraft.kak` kakoune plugin

`buffercraft.kak` sets up some commands to interact with the `kak-buffercraft`
binary depending on the current buffer name.

This is configured with hooks instead of a JSON file as is done in
vim-projectionist. While it wouldn't be super difficult to use JSON instead
of kakoune configuration, it'd take on more dependencies then there's a
question of where to put the JSON file. I prefer controlling all of that
configuration directly in my `kakrc`.

The hook configuration might look like this:

```kak
hook global BufCreate .*[.]ex %{
  set-option buffer buffercraft_kind "lib"
  set-option buffer buffercraft_pattern "lib/(.*)\.ex"
  set-option buffer buffercraft_alternate "test/{{ matches[1] }}_test.exs"
  set-option buffer buffercraft_template \
'defmodule {{ matches[1] | camelcase | capitalize | dot }} do
  @moduledoc """
  """
end'
}
```

The _kind_ is used to select which sort of template and alternate rules you
want to use. The _pattern_ matches the pattern described above, and the
_alternate_ and _template_ are both templates as described above.

## TODO

- figure out how to test stuff in rust
    - this will probably also mean splitting out the main/0 function into
      helper functions
    - set up actions CI for these tests
    - I assume it's not possible to test kakounescript but I'd be pleasantly
      surprised if we can get some automated test cases through the editor
- improve error case handling
    - something something get the error message into the debug buffer
    - maybe it's good enough as it is?
- possibly refactor how configuration is done
    - the hook regex for buffer creation takes the basename of the file,
      not the whole buffile value which means we can't detect special cases
      like Phoenix controllers
    - I'd like to avoid projectionist-style JSON configurations (see mor
      on that below)

## Differences to vim-projectionist

The primary difference is configuration. Buffercraft uses kakoune hooks and
values to drive configuration while vim-projectionist uses a JSON file or
a vim data structure. Kakounescript doesn't support the rich data structures
vimscript does and I found the `.projections.json` to be clunky: do I check
it into git? Do I have to check it in across all repositories?

Buffercraft also does not auto-inject the template into a new buffer for a
file that does not yet exist. This is more of a practical choice since
`echo -to-file` exists and I could not figure out an easy way to echo
directly into the buffer contents (perhaps the `|` pipe operator would help
here). This is a bit of a limitation because the file needs to exist for
this to succee. I'd prefer to have the newly created buffer have the
rendered template contents and then the user can decide if and how they
want to save the file.

## Naming

It sounds like hovercraft and that brings me joy :flying_saucer:
