# rust-cascading-errors

This small project shows how errors can cascade in rust. Missing a single bracket, can lead to 7 errors in this case, and 16 in the actual project this sample code comes from.

This:
```
use serde::{Deserialize, Serialize//};
```

Leads to:

```
    Checking errortest v0.1.0 (/home/frederik/Programmeren/rust/errortest)
error: this file contains an unclosed delimiter
  --> src/noderef.rs:26:3
   |
1  | use serde::{Deserialize, Serialize//};
   |            - unclosed delimiter
...
26 | }
   |   ^

error: expected identifier, found keyword `use`
 --> src/noderef.rs:2:1
  |
2 | use std::net::SocketAddr;
  | ^^^ expected identifier, found keyword

error: expected one of `,`, `::`, `as`, or `}`, found keyword `use`
 --> src/noderef.rs:2:1
  |
1 | use serde::{Deserialize, Serialize//};
  |            - unclosed delimiter   - expected one of `,`, `::`, `as`, or `}`
2 | use std::net::SocketAddr;
  | ^^^ unexpected token
  |
help: `}` may belong here
  |
1 | use serde::{Deserialize, Serialize}//};
  |                                   ^
help: missing `,`
  |
1 | use serde::{Deserialize, Serialize,//};
  |                                   ^

error: expected one of `,`, `::`, `as`, or `}`, found `std`
 --> src/noderef.rs:2:5
  |
2 | use std::net::SocketAddr;
  |    -^^^ expected one of `,`, `::`, `as`, or `}`
  |    |
  |    help: missing `,`

error: expected one of `,`, `::`, `as`, or `}`, found `;`
 --> src/noderef.rs:2:25
  |
2 | use std::net::SocketAddr;
  |                         ^ expected one of `,`, `::`, `as`, or `}`

error: expected one of `*`, `::`, `;`, `{`, or `}`, found `#`
 --> src/noderef.rs:4:1
  |
2 | use std::net::SocketAddr;
  |                          - expected one of `*`, `::`, `;`, `{`, or `}`
3 | 
4 | #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
  | ^ unexpected token
  |
  = note: glob-like brace syntax must be last on the path

error[E0432]: unresolved import `noderef::NodeRef`
 --> src/main.rs:2:5
  |
2 | use noderef::NodeRef;
  |     ^^^^^^^^^^^^^^^^ no `NodeRef` in `noderef`

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0432`.
error: could not compile `errortest`

To learn more, run the command again with --verbose.
```
