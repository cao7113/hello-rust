# https://github.com/casey/just
# https://github.com/casey/just/blob/master/examples/cross-platform.just
# https://just.systems/man/en/
# just -l # list all recipes by: 
# just --help
# just --choose

# Running just with no arguments runs the first recipe in the justfile
# first recipe is the default recipe, this line also acts as RECIPE desc
default:
  @just --list --unsorted --justfile {{justfile()}} # --list-heading $'Customized header line\n' --list-prefix "..."
version:
  just --version
ls-vars:
  just --evaluate

# just prints each command to standard error before running it, which is why echo 'This is a recipe!' was printed.
hi:
	echo hi justfile
# This is suppressed for lines starting with @, which is why echo 'This is another recipe.' was not printed.
hi2:
  # this comment will appeared in recipe
  @echo hi another echo

home_dir := env_var('HOME')
@test-env:
  echo "HOME env-var is: {{home_dir}}"

hex:
  echo {{choose('64', HEX)}}

alias b := build
build:
  @echo 'Building!'


[positional-arguments]
@fox bar:
  echo $0
  echo bar is {{bar}} or $bar or $1
up:
   @echo uppercase: {{uppercamelcase('hi justfile')}}

# Export all variables as environment variables.
# set export
# export variable as environment var
export a := "hello"
export host := `uname -a`

@foo:
  echo $a
  echo $HOME
  echo $host

@info:
  echo "This is an {{arch()}} arch machine, {{num_cpus()}} cpus os: {{os()}} os-family: {{os_family()}}".

# Test vars
with-args *args='':
  @echo 'got args: {{args}}'

# shebang
shebang:
  #!/usr/bin/env bash
  set -euxo pipefail
  hello='Yo'
  echo "$hello from Bash!"

# show recipe definition
show recipe:
  just --show {{recipe}}