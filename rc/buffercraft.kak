declare-option str buffercraft_kind
declare-option str buffercraft_pattern
declare-option str buffercraft_alternate
declare-option str buffercraft_template ""

define-command -docstring "edit the alternate of the current buffer" \
buffercraft-alternate %{
  execute-keys '<esc>: edit %sh{printf %s "$kak_opt_buffercraft_alternate" | kak-buffercraft "$kak_opt_buffercraft_pattern" "$kak_buffile"}<ret>'
}

define-command -docstring "write the buffer template to the current filename" \
buffercraft-template %{
  execute-keys '<esc>: echo -to-file %val{buffile} %sh{printf %s "$kak_opt_buffercraft_template" | kak-buffercraft "$kak_opt_buffercraft_pattern" "$kak_buffile"}<ret>'
}
