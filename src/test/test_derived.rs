use {
    crate::*,
    serde::{
        de::Error,
        Deserialize,
        Deserializer,
    },
    std::collections::HashMap,
};

// allows writing vo!["a", "b"] to build a vec of strings
macro_rules! vo {
    ($($item:literal),* $(,)?) => {{
        let mut vec = Vec::new();
        $(
            vec.push($item.to_owned());
        )*
        vec
    }}
}

// allows writing mo!{"a":"b", "c":"d"} to build a map of strings to strings
macro_rules! mo {
    ($($key:literal:$value:literal),* $(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key.to_owned(), $value.to_owned());
        )*
        map
    }}
}

// this example tries to test all the hard things of Hjson
#[test]
fn test_struct() {
    #[derive(PartialEq, Debug)]
    enum Enum {
        A,
        B,
    }
    // read "a" or "A" as A and "b" or "B" as B
    impl<'de> Deserialize<'de> for Enum {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where D: Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            let s = s.to_lowercase();
            match s.as_ref() {
                "a" => Ok(Enum::A),
                "b" => Ok(Enum::B),
                _ => Err(D::Error::custom(format!("unrecognized enum variant: {:?}", s))),
            }

        }
    }
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
        int: i32,
        float: f64,
        txt1: Option<String>,
        txt2: Option<String>,
        txt3: String,
        seq: Vec<String>,
        enum_map: HashMap<String, Enum>,
    }
    let hjson = r#"
    {
        # Hjson accepts several types of comments.
        /**
         * even the ugly java ones!
         * @WhatAmIDoingHere
         */

        // quotes around keys are optional
        "int": -1 # this comment goes to end of line
        txt2: a quoteless string : with a colon!
        txt3:
            '''
            you can have multiline strings
            and they're free of unexpected spacing
            '''

        // Hjson accepts trailing commas
        seq : [
            another quoteless string
            "b1\nb2",
            "c",
        ]

        # order of keys doesn't matter and you can
        # have a single value after a map
        float: -5.7

        enum_map: {
            "some key"    : a
            "another key" : B
        }
    }
    "#;
    let mut enum_map = HashMap::new();
    enum_map.insert("some key".to_owned(), Enum::A);
    enum_map.insert("another key".to_owned(), Enum::B);
    let expected = Test {
        int: -1,
        float: -5.7,
        txt1: None,
        txt2: Some("a quoteless string : with a colon!".to_owned()),
        txt3: "you can have multiline strings\nand they're free of unexpected spacing".to_owned(),
        seq: vo!["another quoteless string", "b1\nb2", "c"],
        enum_map,
    };
    assert_eq!(expected, from_str(hjson).unwrap());
}

#[test]
fn test_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let j = r#""Unit""#;
    let expected = E::Unit;
    assert_eq!(expected, from_str(j).unwrap());

    let j = r#"{Newtype:1}"#;
    let expected = E::Newtype(1);
    assert_eq!(expected, from_str(j).unwrap());

    let j = r#"
    {
        Tuple : [ # Tuple variant
            1
            2
        ]
    }
    "#;
    let expected = E::Tuple(1, 2);
    assert_eq!(expected, from_str(j).unwrap());

    let j = r#"
    {
        # this variant is explitely defined
        Struct: {a:1}
    }"#;
    let expected = E::Struct { a: 1 };
    assert_eq!(expected, from_str(j).unwrap());
}

#[test]
fn test_arr_struct_untagged() {
    // this enum is untagged: the variant is automatically recognized
    #[derive(Deserialize, PartialEq, Debug)]
    #[serde(untagged)]
    enum Untagged {
        String(String),
        Array(Vec<String>),
    }
    #[derive(Deserialize, PartialEq, Debug)]
    struct InnerThing {
        name: String,
        untagged: Untagged,
    }
    #[derive(Deserialize, PartialEq, Debug)]
    struct OuterThing {
        outer_name: String,
        items: Vec<InnerThing>,
    }
    let hjson = r#"
        {
            outer_name: the thing
            items: [
                {
                    name: first item
                    untagged: "xterm -e \"nvim {file}\""
                }
                {
                    name: "also an \"item\""
                    untagged: ["bla", "et", "bla"]
                }
            ]
        }
    "#;
    let outer_thing = OuterThing {
        outer_name: "the thing".to_owned(),
        items: vec![
            InnerThing {
                name: "first item".to_owned(),
                untagged: Untagged::String("xterm -e \"nvim {file}\"".to_string()),
            },
            InnerThing {
                name: r#"also an "item""#.to_owned(),
                untagged: Untagged::Array(vo!["bla", "et", "bla"]),
            },
        ],
    };
    assert_eq!(outer_thing, from_str::<OuterThing>(hjson).unwrap());
}

#[test]
fn test_string() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct W {
        c: String,
    }
    assert_eq!(W{c:"test".to_string()}, from_str("{c:test\n}").unwrap());
    assert_eq!(W{c:"test".to_string()}, from_str("{c:\"test\"}").unwrap());
    assert_eq!(
        W {c:"xterm -e \"vi /some/path\"".to_string()},
        from_str(r#"{
            c: "xterm -e \"vi /some/path\""
        }"#).unwrap(),
    );
    assert_eq!(W{c:"\x0C\x0C".to_string()}, from_str("{c:\"\\f\\u000C\"}").unwrap());
}

#[test]
fn test_weird_map_keys() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct W {
        map: HashMap<String, String>,
    }
    let hjson = r#"{
        map: {
            <none>: 0
            // π: 3.14
            τ: 6.28
            /: slash // hard one
            \: "" // no trap here
        }
    }"#;
    let value = W {
        map: mo!{
            "<none>": "0",
            "τ": "6.28",
            "/": "slash // hard one", // quoteless string values go til line end
            "\\": "",
        },
    };
    assert_eq!(value, from_str(hjson).unwrap());
}
