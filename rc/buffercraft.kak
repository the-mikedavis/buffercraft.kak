# provide-module buffercraft %{

declare-option str buffercraft_kind
declare-option str buffercraft_pattern
declare-option str buffercraft_alternate
declare-option str buffercraft_template ""

define-command -docstring "edit the alternate of the current buffer" \
buffercraft-alternate %{
  execute-keys '<esc>: edit %sh{echo "$kak_opt_buffercraft_alternate" | kak-buffercraft "$kak_opt_buffercraft_pattern" "$kak_buffile"}<ret>'
}

define-command -docstring "write the buffer template to the current filename" \
buffercraft-template %{
  execute-keys '<esc>: echo -to-file %val{buffile} %sh{printf %s "$kak_opt_buffercraft_template" | kak-buffercraft "$kak_opt_buffercraft_pattern" "$kak_buffile"}<ret>'
}

# }

## example usage

hook global BufCreate mix.exs %{
  set-option buffer buffercraft_kind "mix"
  set-option buffer buffercraft_alternate "mix.lock"
}

hook global BufCreate mix.lock %{
  set-option buffer buffercraft_kind "lock"
  set-option buffer buffercraft_alternate "mix.exs"
}

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
  set-option buffer buffercraft_pattern "test/(.*)_test.exs"
  set-option buffer buffercraft_alternate "lib/{{ matches[1] }}.ex"
  set-option buffer buffercraft_template '
defmodule {{ matches[1] | pascalcase | dot }}Test do
  use ExUnit.Case, async: true
end
  '
}
