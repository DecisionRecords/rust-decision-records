# Decision Records Rust Application

_**WARNING THIS IS A VERY EARLY ALPHA! USE ACCORDINGLY!**_

This application is intended to automate the process of adding and updating Decision Records
(DR), sometimes known as "Architectural Decision Records" (ADR). A DR is a simple file which
explains why a decision was made at a particular time, the context in which that decision was
made, and what implications it may have. These files reside in a specific directory, and have
a specific naming structure. Linking between documents is recommended, and replacing documents
should be fairly commonplace.

## Running the script

You are expected to run `decision-record init` to create a directory structure, then
`decision-record new Decision to use foo` which will create a file called
`doc/decision_records/0001-decision-to-use-foo.md` which produces a specific template. You may
choose to then run `decision-record new --supersede 1 Decision to use bar instead of foo`
which will create the file `doc/decision_records/0002-decision-to-use-bar-instead-of-foo.md`
which has a link showing that this record supersedes the previous record.

Additional options will be available in the help, found when you run `decision-record help`.

## Language support and file paths

There is a configuration file format you can use in any root directory, similar in concept to the
.gitconfig or .vscode file, which is called `.decisionrecords-config`. This file can replace
default paths for:

* The records directory:
  * Default `doc/decision_records`
  * Configure `records=new relative path` to change to "new relative path"
* The path to the templates:
  * Default `$(records)/.templates`
  * Configure `templatedir=relative/path/to/template directory` to change it to this new path
* The type of template file we can use:
  * Default `md`
  * Configure `filetype=rst` to change to Restructured Text.
  * Options: Currently, only `rst` and `md` are supported. If other templates are available,
      please raise a PR to support them!
* The name of the template file to use:
  * Default `template`
  * Configure `template=decision record template` to change the file prefix (excluding language
      and format) to this new path.
* The language of the template and string replacements to use:
  * Default `en`
  * Configure `language=zh-CN` to use Chinese with Simplified Characters, `language=de_DE` to use
              "Standard German", or `language=fr` to use French with no country localization.
  * Notes: This language field should be represented using the ISO-639-1 code for the langauge,
      e.g. `en`, then if a dialect is to be selected, add an underscore or hyphen then the
      dialect code, using the ISO-3166-1 Alpha 2 code for the country, e.g. `GB`. For more
      examples, see [the wikipedia page on Language
      Localisation](https://en.wikipedia.org/wiki/Language_localisation).  This configuration
      relies on the provision of relevant template and translation strings. If a language is
      defined, but not available, the script will fall-back to English.

## Templates

The template file should be stored, according to the language block just mentioned, and needs to
have some key strings stored for exchange. These values are:

* `NUMBER`: This string is replaced with the integer value of the record, e.g. 1, 57 or 999
* `TITLE`: This is the string provided as the title of the new record.
* `DATE`: This is the date that the DR was created, and is stored in a YYYY-MM-DD format.
* `STATUS`: This is the INITIAL value of the status, which defaults to "Approved", but can
   be overriden with `decision-record.sh new -P Some Title` to create a "Proposed record"
   with the title "Some Title", or `decision-record.sh new -S WIP Some Title` to create a
   decision record with a status of "WIP" and a title of "Some Title".

When the titles are parsed (when a record is superseded, linked, amended or deprecated), the
record will be parsed looking for the first of the following values:

* A string matching the regular expression `^\s*# [0-9]+\. .*$`
* A string matching the regular expression `^\s*title=.*$`

This will be injected into the link text whenever a record link is performed.

## Inspiration

This script is inspired by the [ADR-Tools](https://github.com/npryce/adr-tools) which appear to
have been abandoned. This also intends to extend ADR-Tools beyond it's current limitations, and
has the capability to provide additional language support, some common "switches" around new
records, and has a full set of testing, using the built-in test functionality and BATS for
functional testing.

In addition, the longer-term aim for this is to be functionally the SAME as using the similar,
as-yet unreleased scripts written in
[Javascript](https://github.com/DecisionRecords/javascript-decision-records), or in 
[Bash](https://github.com/DecisionRecords/bash-decision-records).

---
**Licensed under the Zero-Clause BSD License (0BSD)**

Permission to use, copy, modify, and/or distribute this software for any purpose with or without
fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
OF THIS SOFTWARE.
