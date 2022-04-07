# zine

`Option<T>.as_ref()` 将会转化为 `Option<&T>`

```rust
fn main() {
    let name: Option<String> = Some("zine".to_string());
    let dir = if let Some(name) = name.as_ref() {
        env::current_dir()?.join(name)
    } else {
        env::current_dir()?
    };
}
```

## OnceCell
https://docs.rs/once_cell/latest/once_cell/

## Lazy
https://docs.rs/once_cell/latest/once_cell/

## RwLock
阅读[RwLock](https://docs.rs/parking_lot/latest/parking_lot/type.RwLock.html)

```rust
fn main() {
    use parking_lot::RwLock;

    let lock = RwLock::new(5);

// many reader locks can be held at once
    {
        let r1 = lock.read();
        let r2 = lock.read();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // read locks are dropped at this point

// only one write lock may be held, however
    {
        let mut w = lock.write();
        *w += 1;
        assert_eq!(*w, 6);
    } // write lock is dropped here
}
```

## BTreeMap

## walkdir

 递归并发遍历复制目录里面所有文件：
```rust
use anyhow::Result;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{fs, io::Read, path::Path};
/// Copy directory recursively.
/// Note: the empty directory is ignored.
pub fn copy_dir(source: &Path, dest: &Path) -> Result<()> {
    let source_parent = source.parent().expect("Can not copy the root dir");
    walkdir::WalkDir::new(source)
        .into_iter()
        .par_bridge()
        .try_for_each(|entry| {
            let entry = entry?;
            let path = entry.path();
            // `path` would be a file or directory. However, we are
            // in a rayon's parallel thread, there is no guarantee
            // that parent directory iterated before the file.
            // So we just ignore the `path.is_dir()` case, when coming
            // across the first file we'll create the parent directory.
            if path.is_file() {
                if let Some(parent) = path.parent() {
                    let dest_parent = dest.join(parent.strip_prefix(source_parent)?);
                    if !dest_parent.exists() {
                        // Create the same dir concurrently is ok according to the docs.
                        fs::create_dir_all(dest_parent)?;
                    }
                }
                let to = dest.join(path.strip_prefix(source_parent)?);
                fs::copy(path, to)?;
            }

            anyhow::Ok(())
        })?;
    Ok(())
}

```

## serde 相关

## rayon
阅读[ParallelBridge)(https://docs.rs/rayon/1.0.3/rayon/iter/trait.ParallelBridge.html)文档。可以很方便使用并行。

除了使用在传统的`Iterator`, 还可以使用在`channels or file or network I/O`.

```rust
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;

fn main() {
    let rx = {
        let (tx, rx) = channel();

        tx.send("one!");
        tx.send("two!");
        tx.send("three!");

        rx
    };

    let mut output: Vec<&'static str> = rx.into_iter().par_bridge().collect();
    output.sort_unstable();

    assert_eq!(&*output, &["one!", "three!", "two!"]);
}
```

## notify

[notify](https://docs.rs/notify/latest/notify/)

```rust
extern crate notify;

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("/home/test/notify", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           Ok(event) => println!("{:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
```