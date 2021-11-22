//! Structs and methods to deal with anchors
//!

// We have the following call tree:
//
//           ----- /anchors                ----- /list  ----- List<A>
//                                         ----- /get  ----- A

/// All operations available
#[derive(Debug)]
enum Ops {
    Get,
    List,
}

/// Generate the proper URL for the service we want in the given category
fn set_url(ops: Ops, uuid: String) -> String {
    match ops {
        Ops::Get => format!("/anchors/{}/", uuid),                         // /get
        Ops::List => "/anchors/".to_string(),                                 // /list
    }
}

