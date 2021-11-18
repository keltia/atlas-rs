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