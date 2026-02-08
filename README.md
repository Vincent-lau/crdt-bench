# CRDT benchmarks

To run the benchmark do

```sh
cargo bench
```

Or if you want to run a particular benchmark then. See the `benches` directory
for what is available.

```sh
cargo bench seq_add
```

Workloads:

- seq_add: This is simulating insertion of large chunk of texts, either one by
one or one (such as user typing characters in a text editor), or one large chunk 
at a time (user pasting a big text). And measure the time it takes to do local
insertion, as well as applying these updates to a second document.
- conc_add: similar to seq_add, but with more than one clients.
- load: The time it takes to load a saved document from disk.
- mem: The memory usage when loading a document from disk.
- version: measuing the time it takes to diff versions of documents and also
reverting to a previous version.

Future plans:

- Add more CRDT libraries than just Automerge and Yrs
- Add more real world workloads

