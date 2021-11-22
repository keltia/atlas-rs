//! Structs and methods to deal with anchor measurements
//!

// We have the following call tree:
//
// Atlas API ----- /anchor-measurement     ----- /list  ----- List<AM>
//                                         ----- /get  ----- AM

/// All operations available
#[derive(Debug)]
enum Ops {
    Get,
    List,
}

/// Generate the proper URL for the service we want in the given category
fn set_url(ops: Ops, uuid: String) -> String {
    match ops {
        Ops::Get => format!("/anchor-measurements/{}/", uuid),                         // /get
        Ops::List => format!("/anchor-measurements/"),                                 // /list
    }
}

