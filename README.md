# buffercraft.kak

Buffercraft is a plugin for the [kakoune](https://github.com/mawww/kakoune)
text editor which tries to emulate some behavior of
[`tpope/vim-projectionist`](https://github.com/tpope/vim-projectionist).

# Installation

If you use [nix](https://nixos.org/) for package management, you may
install the `kak-buffercraft` binary by cloning this repository and
running `nix-env -i -f default.nix`. If you know what you're doing
with flakes, you can also install this as a flake (see the `flake.nix`).

If you're just using `cargo` to build the rust part of this project,

```
cargo install
```

will install the `kak-buffercraft` binary into your cargo binaries path.

With the `kak-buffercraft` binary installed, now add the kakoune plugin.
You can drop in the `rc/buffercraft.kak` into your kakoune auto-load
directory ([see the kakoune wiki on plugin
installation](https://github.com/mawww/kakoune/wiki/Installing-Plugins),
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
`lib/my_app/my_module.ex` would be the _prospect_ (first command-line
argument) and `lib/(.*)\.ex` would be the _pattern_ (second command-line
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
hook global BufCreate lib/.*\.ex %{
  set-option buffer buffercraft_kind "lib"
  set-option buffer buffercraft_pattern "lib/(.*)\.ex"
  set-option buffer buffercraft_alternate "test/{{ matches[1] }}_test.exs"
  set-option buffer buffercraft_template '
  defmodule {{ matches[1] | camelcase | capitalize | dot }} do
    @moduledoc """
    """
  end
  '
}
```

The _kind_ is used to select which sort of template and alternate rules you
want to use. The _pattern_ matches the pattern described above, and the
_alternate_ and _template_ are both templates as described above.

## Naming

It sounds like hovercraft and that brings me joy :hugs:
