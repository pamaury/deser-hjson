<a name="v1.1.0"></a>
### v1.1.0 - 2022-12-21
- support for braceless Hjson - Fix #7

<a name="v1.0.2"></a>
### v1.0.2 - 2021-07-31
- fix tab after quoteless map key being read as part of the key

<a name="v1.0.1"></a>
### v1.0.1 - 2021-06-22
- properly parse single quote strings
- fix type guessing in some cases for null, false, and true

<a name="v1.0.0"></a>
### v1.0.0 - 2021-06-15
- it's stable. Calling it a 1.0

<a name="v0.1.13"></a>
### v0.1.13 - 2021-05-26
- make \r\n behave like \n
- allow more liberty for enum variants

<a name="v0.1.12"></a>
### v0.1.12 - 2021-02-13
- more precise number type guessing

<a name="v0.1.11"></a>
### v0.1.11 - 2021-02-11
- fix primitive types (ie not Hjson texts but primitives like integers and floats) needing a space at the end - Fix #1

<a name="v0.1.10"></a>
### v0.1.10 - 2021-02-11
- make from_str parse a `DeserializeOwned` instead of a borrowed `Deserialize<'a>`
