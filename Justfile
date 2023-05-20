default:
  @just --list --justfile {{justfile()}} --list-heading $'Cool stuff…\n'

hi:
	echo hi justfile

hi1:
  @echo hi another echo

alias b := build

build:
  @echo 'Building!'

set export

a := "hello"

@foo:
  echo $a
  echo $HOME


system-info:
  @echo "This is an {{arch()}} machine, os: {{os()}} os-family: {{os_family()}}".

home_dir := env_var('HOME')

test:
  echo "{{home_dir}}"