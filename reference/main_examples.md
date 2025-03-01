<span style="font-family:'Amiri'">

Just a place for me to store different main.rs programs and examples from this project.
# Main Examples
AWS stuff
```
pub mod aws_stuff;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_dynamodb::Error> {
    let _test = aws_stuff::dynamo_db_ops::list_tables().await;
    Ok(())
}
```

Base components of Task
```
pub mod types;

fn main() {
    let f: types::FrequencyScalar = (1, 2);
    let _test: types::Multiplier = types::Multiplier::Exponential(f);
    let _test = match _test {
        types::Multiplier::Exponential((x, y)) => {
            println!("Exponentiate every {} day(s) \nScale by {}", x, y);
            types::Multiplier::Exponential((x, y))
        },
        types::Multiplier::Scalar(x) => {
            println!("x: {}", x);
            types::Multiplier::Scalar(1)
        },
    };
}
```

Simple initialization and update of a Task
```
pub mod types;
pub mod aws_stuff;
pub mod values;

use types::task::Task;

fn main() {
    let mut _test = Task::new_default();

    _test.print();

    _test.update_priority();

    _test.print();
}
```

Simple initialization and update of a Folder type
```
pub mod types;
pub mod aws_stuff;
pub mod values;

use types::folder::Folder;
use types::task::Task;

fn main() {
    let mut _test_folder = Folder::new();
    let _test_task = Task::new_default();

    _test_folder.add_task(_test_task);
    
    _test_folder.print_tasks();
}

```

</span>