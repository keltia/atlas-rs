//! Structs and methods to deal with credits
//!

//  This has the following call-tree
//
//           ----- /credits                ----- /get
//                                         ----- /get  ----- /incomes
//                                                           /expenses
//                                                           /transfers
//                                                           /transactions
//                                                           /members
//                                                           /members      ----- /claim
//

/// All operations available
#[derive(Debug)]
enum Ops {
    Get = 1,
    Incomes,
    Expenses,
    Transfers,
    Transactions,
    Members,
    Claim,
}

/// Generate the proper URL for the service we want in the given category
fn set_url(ops: Ops) -> String {
    match ops {
        Ops::Get => format!("/credits/"),                         // /get
        Ops::Incomes => format!("/credits/incomes/"),                         // /set
        Ops::Expenses => format!("/credits/expenses/"),                      // /delete
        Ops::Transfers => format!("/credits/transfers/"),         // /list
        Ops::Transactions => format!("/credits/transactions/"),   // /list
        Ops::Members => format!("/credits/members/"),                               // /create
        Ops::Claim => format!("/credits/members/claim/"),                               // /create
        _ => "unsupported",
    }
}

