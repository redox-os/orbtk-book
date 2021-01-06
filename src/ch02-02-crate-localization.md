# The Localization Crate

Localization is a research task by itself, if you want to resolve all syntactic
rules that are found when writing prose in different languages.
OrbTK's localization crate isn't ready to resolve all this complexity, but
this may improve in further releases.

Starting with the given implementation, `localization` can offer methods, that
are able to match and replace text strings. The usage of the `localization` crate is
optional. If you don't need any multi lingual adaptions inside your widgets, simply
do not include the `localization` sugar.

## The building blocks of `localization`

If you want to enable the users to select and change the desired display
language of the GUI at runtime, the toolkit needs to match up a requested
text strings (the key) that should be presented inside the view and substitute
it with the corresponding translation string (the target value). Dictionaries
are used to organize the keys as word lists.

OrbTK's `localization` implementation has choosen to persitently store the
translation strings inside a `ron` file. When introducing the new syntax
structure used inside a `ron` filetype, it was one goal of the authors to
easily match rust types to ron types.

You can save each supported language in its individual ron file. The language
files need to be distinctable. A natural way to implement this requirement
is the usage of unique `language ids`. Most **operating systems* take advantage
of a `locale subsystem`, and save the identification of the active language in
the `lang` enviroment variable. It's good practice to include the language id in
the corresponding ron file name.

When you include the `localization` functionality in your OrbTK code, you
should define constants for each supported `language id`, that will reference the
ron file in question.

When calling the `RonLocalization` methods addressing the combination of a language
id and the corresponding dictionary you are able to store the result in `language`
variable. The crate methods will handle all the heavy lifting to substitute the
source values of the text attributes inside the views with their matching translation
strings in the addressed dictionary.

## The ron file structure

In OrbTK, the structure `RonLocalizationBuilder` is defined to take values for
the following parameters

* language: a String
* dictionaries: a HashMap

The ron filename representing a language localization should include the language
identifier to ease its distiction from another.

Dictionaries itself are stored
The dictionary is represended by a key value pair

A class `Dictionary` will include a `map` named **words**.
The ron type `map` is like a type `struct`**, but keys are also values instead of
just beenig identifiers.

** using a ron file

Activation of the `localization` crate inside your source code boils down this
short example code.

```rust
pub const EN_US: &str = include_str!("../assets/dictionary_en_US.ron");
pub const DE_DE: &str = include_str!("../assets/dictionary_de_DE.ron");
pub const ES_ES: &str = include_str!("../assets/dictionary_es_ES.ron");

let mut localization = RonLocalization::create()
	.language("en_US")
	.dictionary("en_US", EN_US)
	.build();
if let Some(text) = localization.text("hello") {
	println!("{}", text);
}

let localization = RonLocalization::create()
	.language("de_DE")
	.dictionary("de_DE", DE_DE)
	.build();
if let Some(text) = localization.text("hello") {
	println!("{}", text);
}

let localization = RonLocalization::create()
	.language("es_ES")
	.dictionary("es_ES", ES_ES)
	.build();
if let Some(text) = localization.text("hello") {
	println!("{}", text);
}
```

Sure, this code isn't elegant nor will it suite a real applications demands.
What it does show is the logic, to bind a ron file (storing the translations
of a given language) to a const. When calling `RonLocalization`, the `text`
method will resolve text attributes inside a view or any rust primitive with
the translation text resolved in the language dictionary.
