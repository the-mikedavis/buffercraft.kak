provide-module buffercraft %§

declare-option buffer str buffercraft_kind
declare-option buffer str buffercraft_pattern
declare-option buffer str buffercraft_alternate
declare-option buffer str buffercraft_template ""

§

## example usage

hook global BufCreate mix.exs %{
  set-option buffer buffercraft_kind "mix"
  set-option buffer buffercraft_alternate "mix.lock"
}

hook global BufCreate mix.lock %{
  set-option buffer buffercraft_kind "lock"
  set-option buffer buffercraft_alternate "mix.exs"
}

hook global BufCreate lib/.*\.ex %{
  set-option buffer buffercraft_kind "lib"
  set-option buffer buffercraft_pattern "lib/(.*)\.ex"
  set-option buffer buffercraft_alternate "test/{{ match[1] }}_test.exs"
  set-option buffer buffercraft_template '
  defmodule {{ match[1] | camelcase | capitalize | dot }} do
    @moduledoc """
    """
  end
  '
}

hook global BufCreate test/.*_test.exs %{
  set-option buffer buffercraft_kind "test"
  set-option buffer buffercraft_pattern "test/(.*)_test.exs"
  set-option buffer buffercraft_alternate "lib/{{ match[1] }}.ex"
  set-option buffer buffercraft_template '
  defmodule {{ match[1] | camelcase | capitalize | dot }}Test do
    use ExUnit.Case, async: true
  end
  '
}
